use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::domain::entities::bible::BibleBookInfo;
use crate::domain::value_objects::ID;
use crate::app::errors::AppError;
use crate::app::gateways::abstract_bible::BibleGateway;
use crate::adapters::schemas::bible::BibleBookInfoSchema;

pub struct PostgresBibleGateway {
    db: Arc<Pool<Postgres>>,
}

impl PostgresBibleGateway {
    pub fn new(db: Arc<Pool<Postgres>>) -> Self {
        PostgresBibleGateway { db }
    }
}

#[async_trait]
impl BibleGateway for PostgresBibleGateway {
    
    async fn list_books(&self, tr_id: ID) -> Result<Arc<Vec<BibleBookInfo>>, AppError> {
        let select_query = sqlx::query_as!(BibleBookInfoSchema, 
            r#"
            SELECT 
                id, no, name, short, is_new_testament, created_at, updated_at
            FROM bible_books WHERE tr_id=$1
            "#, 
            tr_id.0,
        );
	    let query_result = select_query.fetch_all(self.db.as_ref()).await;

        return match query_result {
            Ok(raw_vec) => {
                let bible_books = raw_vec.into_iter()
                    .map(|schema| schema.into())
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