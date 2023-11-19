use std::sync::Arc;
use async_trait::async_trait;

use crate::domain::exceptions::DomainError;
use crate::domain::entities::bible::{BibleVerse, RefRange};

#[async_trait]
pub trait BibleGateway: Send + Sync {
    /// Returns list of BibleVerse in specified reference range and language
    async fn get_verses_by_range<'a>(
        &self, range: Arc<RefRange<'a>>, lang_code: &str,
    ) -> Result<Arc<Vec<BibleVerse>>, DomainError>;
}
