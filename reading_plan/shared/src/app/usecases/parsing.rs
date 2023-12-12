use std::sync::Arc;
use async_trait::async_trait;

use crate::domain::entities::bible::ExcerptRef;
use crate::app::errors::AppError;

#[async_trait]
pub trait ParseReference: Send + Sync {
    /// 
    /// Parses reference line to get the beginning and the end of a fragment.
    /// 
    /// Full format:
    /// BookA[_<ch>[:<vers>]][-BookB[_<ch>[:<vers>]]]
    /// 
    /// Reference formats:
    /// - "Кор" -> whole book content 
    /// - "Кор_1" -> whole book chapter's content
    /// - "Кор_1:3" -> specifiec verse
    /// - "Кор_1" 
    /// - "Кор_1-Флс_2:3"
    /// 
    async fn parse_reference(
        &self, reference: &str
    ) -> Result<Arc<ExcerptRef>, AppError>;
}

pub trait CleanReference: Send + Sync {
    ///
    /// Replaces extra spaces, space-substitutes with single space char
    /// and returns cleaned ref line.
    /// 
    fn clean_ref_line(line: &str) -> String;
}
