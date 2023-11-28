use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::domain::entities::translation::Translation;
use crate::app::errors::AppError;
use crate::app::gateways::abstract_translation::TranslationGateway;
use crate::adapters::schemas::translation::TranslationSchema;

pub struct PostgresTranslationGateway {
    db: Arc<Pool<Postgres>>,
}

impl PostgresTranslationGateway {
    pub fn new(db: Arc<Pool<Postgres>>) -> Self {
        PostgresTranslationGateway { db }
    }
}

#[async_trait]
impl TranslationGateway for PostgresTranslationGateway {
    
    async fn list_translations(&self) -> Result<Arc<Vec<Translation>>, AppError> {
        let select_query = sqlx::query_as!(TranslationSchema, 
            r#"
            SELECT id, lang_id, name FROM bible_translations
            "#        
        );
	    let query_result = select_query.fetch_all(self.db.as_ref()).await;

        return match query_result {
            Ok(raw_vec) => {
                let translations = raw_vec.into_iter()
                    .map(|schema| schema.into())
                    .collect::<Vec<Translation>>();
                Ok(Arc::new(translations))
            },
            Err(err) => {
                println!("{:?}", err);
                Err(AppError::Base)
            }
        }
    }
}