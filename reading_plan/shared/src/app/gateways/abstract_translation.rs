use std::sync::Arc;
use async_trait::async_trait;

use crate::domain::entities::translation::Translation;
use crate::app::errors::AppError;

#[async_trait]
pub trait TranslationGateway: Send + Sync {
    /// Returns list of Translations
    async fn list_translations(&self) -> Result<Arc<Vec<Translation>>, AppError>;
}
