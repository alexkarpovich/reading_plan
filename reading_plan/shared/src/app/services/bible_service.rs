use std::sync::Arc;
use async_trait::async_trait;

use crate::app::usecases::bible::ListBooks;
use crate::domain::{
    entities::bible::BibleBookInfo,
    value_objects::ID,
};
use crate::app::errors::AppError;
use crate::app::gateways::{
    abstract_parsing::ParseReference,
    abstract_bible::BibleGateway,
};

#[derive(Clone)]
pub struct BibleService {
    //parse_gateway: Arc<dyn ParseReference>,
    bible_gateway: Arc<dyn BibleGateway>,
}

impl BibleService {
    pub fn new(
        bible_gateway: Arc<dyn BibleGateway>, 
        _parse_gateway: Arc<dyn ParseReference>,
    ) -> Self {
        Self{bible_gateway/* , parse_gateway*/}
    }
}

#[async_trait]
impl ListBooks for BibleService {
    async fn list_books(&self, tr_id: ID) -> Result<Arc<Vec<BibleBookInfo>>, AppError> {
        let books = self.bible_gateway.get_books(tr_id).await?;
        Ok(books)
    }
}
