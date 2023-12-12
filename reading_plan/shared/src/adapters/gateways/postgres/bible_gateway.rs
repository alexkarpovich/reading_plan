use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{
    Pool, 
    Postgres, 
};

use crate::domain::entities::bible::{
    BibleBook, BibleContentWithBook, BibleBookInfo,
};
use crate::domain::value_objects::ID;
use crate::app::errors::AppError;
use crate::app::usecases::gateways::{
    ListBooksGateway, GetExcerptGateway, BibleGateway, 
};

pub struct PostgresBibleGateway {
    db: Arc<Pool<Postgres>>,
}

impl PostgresBibleGateway {
    pub fn new(db: Arc<Pool<Postgres>>) -> Self {
        PostgresBibleGateway { db }
    }
}

impl BibleGateway for PostgresBibleGateway {}

#[async_trait]
impl ListBooksGateway for PostgresBibleGateway {
    
    async fn list_books(&self, tr_id: ID) -> Result<Arc<Vec<BibleBookInfo>>, AppError> {
        let select_query = sqlx::query!(
            r#"
            SELECT 
                id, no, key, name, is_new_testament, created_at, updated_at
            FROM bible_books WHERE tr_id=$1
            "#, 
            tr_id.0,
        );
	    let query_result = select_query.fetch_all(self.db.as_ref()).await;

        return match query_result {
            Ok(row_vec) => {
                let bible_books = row_vec.into_iter()
                .map(|row| BibleBookInfo {
                    id: ID(row.id),
                    no: row.no as u8,
                    key: row.key.to_owned(),
                    name: row.name.to_owned(),
                    is_new_testament: row.is_new_testament,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                })
                .collect::<Vec<BibleBookInfo>>();
                Ok(Arc::new(bible_books))
            },
            Err(err) => {
                println!("{:?}", err);
                Err(AppError::Base)
            }
        }
    }
}

#[async_trait]
impl GetExcerptGateway for PostgresBibleGateway {

    async fn list_excerpt_books(&self, tr_id: ID, start_book_alias: &str, end_book_alias: &str) -> Result<Vec<BibleBook>, AppError> {
        let select_query = sqlx::query!( 
            r#"
            SELECT id, no, key, name, is_new_testament, created_at, updated_at 
            FROM bible_books 
            WHERE tr_id=$1 
                AND no >= (
                    SELECT no FROM bible_books 
                    WHERE tr_id=$1 AND (name=$2 OR $2=ANY(aliases))
                ) 
                AND no <= (
                    SELECT no FROM bible_books 
                    WHERE tr_id=$1 AND (name=$3 OR $3=ANY(aliases))
                ) 
            "#,
            tr_id.0,
            start_book_alias,
            end_book_alias,
        );
	    let query_result = select_query.fetch_all(self.db.as_ref()).await;

        return match query_result {
            Ok(row_vec) => {
                let books = row_vec.into_iter()
                    .map(|row| BibleBook {
                        id: ID(row.id),
                        no: row.no as u8,
                        key: row.key.to_owned(),
                        name: row.name.to_owned(),
                        content: vec![],
                        is_new_testament: row.is_new_testament,
                        created_at: row.created_at,
                        updated_at: row.updated_at,
                    })
                    .collect::<Vec<BibleBook>>();
                Ok(books)
            },
            Err(err) => {
                println!("{:?}", err);
                Err(AppError::Base)
            }
        }
    }

    async fn list_excerpt_content(&self, tr_id: ID, start_key: &str, end_key: &str) -> Result<Vec<BibleContentWithBook>, AppError> {
        let select_query = sqlx::query!( 
            r#"
            SELECT id, book_id, chapter, vers, text
            FROM content 
            WHERE tr_id=$1 and key >= $2 and key <= $3
            "#,
            tr_id.0,
            start_key,
            end_key,
        );
	    let query_result = select_query.fetch_all(self.db.as_ref()).await;

        return match query_result {
            Ok(row_vec) => {
                let content_vec = row_vec.into_iter()
                    .map(|row| BibleContentWithBook {
                        id: ID(row.id.unwrap()),
                        book_id: ID(row.book_id.unwrap()),
                        chapter: row.chapter.unwrap() as u8,
                        vers: row.vers.unwrap() as u8,
                        text: row.text.unwrap(),
                    })
                    .collect::<Vec<BibleContentWithBook>>();
                Ok(content_vec)
            },
            Err(err) => {
                println!("{:?}", err);
                Err(AppError::Base)
            }
        }
    }
}
