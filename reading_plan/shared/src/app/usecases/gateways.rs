use std::sync::Arc;
use async_trait::async_trait;

use crate::domain::value_objects::ID;
use crate::domain::entities::bible::{BibleBookInfo, BibleBook, BibleContentWithBook};
use crate::domain::entities::translation::Translation;
use crate::app::errors::AppError;


#[async_trait]
pub trait ListBooksGateway: Send + Sync {
    /// Returns list of BibleBookInfo for specified translation
    async fn list_books(&self, tr_id: ID) -> Result<Arc<Vec<BibleBookInfo>>, AppError>;

}

#[async_trait]
pub trait GetExcerptGateway: Send + Sync {
    /// Returns list of books for specific fragment
    async fn list_excerpt_books(&self, tr_id: ID, start_book_alias: &str, end_book_alias: &str) -> Result<Vec<BibleBook>, AppError>;

    /// Returns content for specific excerpt
    async fn list_excerpt_content(&self, tr_id: ID, start_key: &str, end_key: &str) -> Result<Vec<BibleContentWithBook>, AppError>;
}

#[async_trait]
pub trait ListTranslationGateway: Send + Sync {
    /// Returns list of Translations
    async fn list_translations(&self) -> Result<Arc<Vec<Translation>>, AppError>;
}
