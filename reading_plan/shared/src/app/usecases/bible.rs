use std::sync::Arc;
use async_trait::async_trait;

use crate::domain::exceptions::DomainError;
use crate::domain::entities::bible::{
    BibleBook,
    BibleVerse,
};

#[async_trait]
pub trait ListBooks {
    
    /// Returns list of bible books for specific language
    async fn list_books(&self, lang_code: &str) -> Vec<BibleBook>;
}

#[async_trait]
pub trait GetVersesByRef {
    /// Returns list of verses for specific language by parsing 
    /// provided reference string
    /// Reference example: `Кор 1:2-5`
    async fn list_verses_by_ref(&self, reference: &str, lang_code: &str) -> Result<Arc<Vec<BibleVerse>>, DomainError>;
}