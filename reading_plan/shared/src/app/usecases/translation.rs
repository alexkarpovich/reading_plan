use std::sync::Arc;
use async_trait::async_trait;

use crate::domain::entities::translation::Translation;
use crate::app::errors::AppError;

#[async_trait]
pub trait ListTranslations {
    /// Returns list of bible translations
    async fn list_translations(&self) -> Result<Arc<Vec<Translation>>, AppError>;
}
