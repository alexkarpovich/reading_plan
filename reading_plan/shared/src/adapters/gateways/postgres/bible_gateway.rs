use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::domain::entities::bible::BibleBookInfo;
use crate::domain::value_objects::ID;
use crate::app::errors::AppError;
use crate::app::gateways::abstract_bible::BibleGateway;
use crate::adapters::schemas::bible::BibleBookInfoSchema;

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
    
    async fn get_books(&self, tr_id: ID) -> Result<Arc<Vec<BibleBookInfo>>, AppError> {
        let query = "SELECT * FROM bible_books WHERE tr_id=$1";

        let select_query = 
            sqlx::query_as::<_, BibleBookInfoSchema>(query)
            .bind(tr_id);
	    let query_result = select_query.fetch_all(self.db.as_ref()).await;

        return match query_result {
            Ok(raw_vec) => {
                let bible_books = raw_vec.iter()
                    .map(|raw| BibleBookInfo {
                        id: raw.id,
                        no: raw.no as u8,
                        name: raw.name.to_owned(),
                        short: raw.short.to_owned(),
                        created_at: raw.created_at,
                        updated_at: raw.updated_at,
                    })
                    .collect::<Vec<BibleBookInfo>>();
                Ok(Arc::new(bible_books))
            },
            Err(_) => Err(AppError::Base)
        }
    }
}