use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::domain::entities::bible::{BibleVerse, RefRange};
use crate::domain::exceptions::DomainError;
use crate::app::gateways::abstract_bible::BibleGateway;

pub struct PostgresBibleGateway {
    db: Arc<Pool<Postgres>>,
}

impl PostgresBibleGateway {
    pub fn new(db: Arc<Pool<Postgres>>) -> Self {
        PostgresBibleGateway { db }
    }
}

#[async_trait]
impl BibleGateway for PostgresBibleGateway {
    async fn get_verses_by_range<'b>(
        &self, _range: Arc<RefRange<'b>>, _lang_code: &str,
    ) -> Result<Arc<Vec<BibleVerse>>, DomainError> {

        let query = "SELECT * FROM bible_verses";

        let query_result = sqlx::query(query)
            .fetch_all(self.db.as_ref()) // -> Vec<{ country: String, count: i64 }>
            .await;

        return match query_result {
            Ok(_) => Ok(Arc::new(vec![])),
            Err(_) => Err(DomainError::InvalidID)
        }
    }
}