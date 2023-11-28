use std::sync::Arc;
use async_trait::async_trait;

use crate::domain::entities::translation::Translation;
use crate::app::usecases::translation::ListTranslations;
use crate::app::errors::AppError;
use crate::app::gateways::abstract_translation::TranslationGateway;


#[derive(Clone)]
pub struct TranslationService {
    translation_gateway: Arc<dyn TranslationGateway>,
}

impl TranslationService {
    pub fn new(
        translation_gateway: Arc<dyn TranslationGateway>, 
    ) -> Self {
        Self{translation_gateway}
    }
}

#[async_trait]
impl ListTranslations for TranslationService {
    async fn list_translations(&self) -> Result<Arc<Vec<Translation>>, AppError> {
        let translations = self.translation_gateway.list_translations().await?;
        Ok(translations)
    }
}
