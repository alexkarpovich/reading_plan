use std::sync::Arc;
use async_trait::async_trait;

use crate::domain::value_objects::ID;
use crate::domain::entities::bible::{BibleBookInfo, BibleExcerpt};
use crate::app::errors::AppError;

#[async_trait]
pub trait ListBooks {
    /// Returns list of bible books info for specific language
    async fn list_books(&self, tr_id: ID) -> Result<Arc<Vec<BibleBookInfo>>, AppError>;
}

#[async_trait]
pub trait GetExcerpt {
    /// Returns bible fragment for specific language
    async fn get_excerpt(&self, tr_id: ID, reference: &str) -> Result<Arc<BibleExcerpt>, AppError>;
}
