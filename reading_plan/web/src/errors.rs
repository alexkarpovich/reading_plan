use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde_json::json;

use shared::app::errors::AppError;
use shared::domain::errors::DomainError;

/// Our app's top level error type.
pub enum Error {
    DomainError(DomainError),
    AppError(AppError),
}

impl From<DomainError> for Error {
    fn from(inner: DomainError) -> Self {
        Error::DomainError(inner)
    }
}

impl From<AppError> for Error {
    fn from(inner: AppError) -> Self {
        Error::AppError(inner)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error::DomainError(DomainError::InvalidID) => {
                (StatusCode::NOT_FOUND, "User not found")
            }
            Error::AppError(AppError::ParseReferenceError) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Parsing reference error")
            }
            _ => (StatusCode::NOT_FOUND, "Unhandled error"),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}