use std::sync::Arc;
use async_trait::async_trait;

use crate::domain::entities::translation::Translation;
use crate::app::usecases::translation::ListTranslations;
use crate::app::errors::AppError;
use crate::app::usecases::gateways::ListTranslationGateway;


#[derive(Clone)]
pub struct ListTranslationsImpl {
    pub translation_gateway: Arc<dyn ListTranslationGateway>,
}

impl ListTranslationsImpl {
    pub fn new(
        translation_gateway: Arc<dyn ListTranslationGateway>, 
    ) -> Self {
        Self{translation_gateway}
    }
}

#[async_trait]
impl ListTranslations for ListTranslationsImpl {
    async fn execute(&self) -> Result<Arc<Vec<Translation>>, AppError> {
        let translations = self.translation_gateway.list_translations().await?;
        Ok(translations)
    }
}
