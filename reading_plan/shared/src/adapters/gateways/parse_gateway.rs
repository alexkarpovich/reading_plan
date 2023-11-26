use std::sync::Arc;
use async_trait::async_trait;

use crate::domain::entities::bible::RefRange;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::ID;
use crate::app::gateways::abstract_parsing::ParseReference;

pub struct ParsingGateway {}

#[async_trait]
impl ParseReference for ParsingGateway {
    async fn parse_reference(&self, value: &str) -> Result<Arc<RefRange>, DomainError> {
        let _ = value;
        Ok(Arc::new(RefRange { start: ID(10), end: ID(11) }))
    }
}