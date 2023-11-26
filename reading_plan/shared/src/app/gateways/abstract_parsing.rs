use std::sync::Arc;
use async_trait::async_trait;

use crate::domain::{entities::bible::RefRange, errors::DomainError};

#[async_trait]
pub trait ParseReference: Send + Sync {
    async fn parse_reference(&self, value: &str) -> Result<Arc<RefRange>, DomainError>;
}
