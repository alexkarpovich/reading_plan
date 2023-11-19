use std::sync::Arc;
use async_trait::async_trait;

use crate::domain::{
    entities::bible::BibleVerse,
    exceptions::DomainError,
};
use crate::app::{
    gateways::{
        abstract_parsing::ParseReference,
        abstract_bible::BibleGateway,
    },
    usecases::bible::GetVersesByRef,
};

pub struct BibleService {
    parse_gateway: Arc<dyn ParseReference>,
    bible_gateway: Arc<dyn BibleGateway>,
}

impl BibleService {
    pub fn new(
        bible_gateway: Arc<dyn BibleGateway>, 
        parse_gateway: Arc<dyn ParseReference>,
    ) -> Self {
        Self{bible_gateway, parse_gateway}
    }
}

#[async_trait]
impl GetVersesByRef for BibleService {
    async fn list_verses_by_ref(&self, reference: &str, lang_code: &str) -> Result<Arc<Vec<BibleVerse>>, DomainError> {
        let range = self.parse_gateway.parse_reference(reference).await?;
        let verses = self.bible_gateway.get_verses_by_range(range, lang_code).await?;
        
        Ok(verses)
    }
}