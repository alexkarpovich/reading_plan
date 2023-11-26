use std::sync::Arc;
use async_trait::async_trait;

use crate::domain::value_objects::ID;
use crate::domain::entities::bible::BibleBookInfo;
use crate::app::errors::AppError;

#[async_trait]
pub trait BibleGateway: Send + Sync {
    /// Returns list of BibleBookInfo for specified translation
    async fn get_books(&self, tr_id: ID) -> Result<Arc<Vec<BibleBookInfo>>, AppError>;
}
