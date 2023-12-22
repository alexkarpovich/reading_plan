use std::sync::Arc;
use async_trait::async_trait;

use crate::app::usecases::bible::{
    ListBooks,
    GetExcerpt,
};
use crate::app::usecases::parsing::ParseReference;
use crate::domain::entities::bible::{BibleBook, BibleContent};
use crate::domain::{
    entities::bible::{BibleBookInfo, BibleExcerpt},
    value_objects::ID,
};
use crate::app::errors::AppError;
use crate::app::usecases::gateways::BibleGateway;


#[derive(Clone)]
pub struct BibleService {
    bible_gateway: Arc<dyn BibleGateway>,
    parsing_service: Arc<dyn ParseReference>,
}

impl BibleService {
    pub fn new(
        bible_gateway: Arc<dyn BibleGateway>,
        parsing_service: Arc<dyn ParseReference>,
    ) -> Self {
        Self{bible_gateway, parsing_service}
    }

    async fn list_excerpt_books(&self, tr_id: ID, start_book_alias: Option<&str>, end_book_alias: Option<&str>) -> Result<Vec<BibleBook>, AppError> {
    
        let start_bk_alias = start_book_alias.as_ref().unwrap();
        let mut end_bk_alias = start_bk_alias;
        
        if let Some(e_bk_alias) = end_book_alias.as_ref() {
            end_bk_alias = e_bk_alias
        }

        let excerpt_books = self.bible_gateway.list_excerpt_books(tr_id, start_bk_alias, end_bk_alias).await.unwrap();

        tracing::info!("Fragment book boundaries: [{:?}]", excerpt_books);

        Ok(excerpt_books)
    }

    fn get_fragment_indexes(
        start_book_no: u8, 
        start_chapter: Option<u8>, 
        start_vers: Option<u8>,
        end_book_no: Option<u8>, 
        end_chapter: Option<u8>, 
        end_vers: Option<u8>,
    ) -> (String, String) {
        let mut start: [u8; 3] = [start_book_no, 1, 1];
        let mut end: [u8; 3] = [start_book_no, u8::MAX, u8::MAX];

        if let Some(end_book_no) = end_book_no {
            end[0] = end_book_no;
        }  


        if let Some(s_ch) = start_chapter {
            start[1] = s_ch;

            let is_same_book = start[0] == end[0];

            if is_same_book {
                end[1] = start[1];
            }

            if let Some(s_vers) = start_vers {
                start[2] = s_vers;

                if is_same_book {
                    end[2] = start[2];
                }
            }
        }
        
        if let Some(e_ch) = end_chapter {
            end[1] = e_ch;

            if let Some(e_vers) = end_vers {
                end[2] = e_vers;
            } else {
                end[2] = u8::MAX;
            }
        }

        (
            format!("{:0>3}.{:0>3}.{:0>3}", start[0], start[1], start[2]), 
            format!("{:0>3}.{:0>3}.{:0>3}", end[0], end[1], end[2]),
        )
    }

    
}

#[async_trait]
impl ListBooks for BibleService {
    async fn list_books(&self, tr_id: ID) -> Result<Arc<Vec<BibleBookInfo>>, AppError> {
        let books = self.bible_gateway.list_books(tr_id).await?;
        Ok(books)
    }
}

#[async_trait]
impl GetExcerpt for BibleService {

    async fn get_excerpt(&self, tr_id: ID, reference: &str) -> Result<Arc<BibleExcerpt>, AppError> {
        let excerpt_ref = self.parsing_service.parse_reference(reference).await.unwrap();
        let start_book_alias = excerpt_ref.0.book.as_ref().unwrap();
        let mut end_book_alias = start_book_alias;
        
        if let Some(e_bk_alias) = excerpt_ref.1.book.as_ref() {
            end_book_alias = e_bk_alias
        }

        let mut books = self.list_excerpt_books(
            tr_id, Some(start_book_alias), Some(end_book_alias),
        ).await.unwrap();

        let start_book = books.first().unwrap();
        let end_book = books.last().unwrap();

        let (start_key, end_key) = BibleService::get_fragment_indexes(
            start_book.no, 
            excerpt_ref.0.chapter, 
            excerpt_ref.0.vers,
            Some(end_book.no),
            excerpt_ref.1.chapter, 
            excerpt_ref.1.vers,
        );

        tracing::info!("Boundary keys: [{start_key}, {end_key}]");

        let mut book_i = 0;
        let mut cur_book = &mut books[book_i];  
        let content_vec = self.bible_gateway.list_excerpt_content(tr_id, &start_key, &end_key).await.unwrap();
        
        for content in &content_vec {
            if content.book_id != cur_book.id {
                book_i += 1;
                cur_book = &mut books[book_i];
            }

            cur_book.content.push(BibleContent { 
                id: content.id,
                chapter: content.chapter, 
                vers: content.vers, 
                text: content.text.to_owned(), 
            });
        }
        
        Ok(Arc::new(BibleExcerpt { books: books }))
    }
}