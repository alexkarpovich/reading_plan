use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde_json::json;

use shared::adapters::errors::AdapterError;
use shared::app::errors::AppError;
use shared::domain::errors::DomainError;

/// Our app's top level error type.
pub enum Error {
    DomainError(DomainError),
    /// Something went wrong when calling the user repo.
    AppError(AppError),
    AdopterError(AdapterError),
}

impl From<DomainError> for Error {
    fn from(inner: DomainError) -> Self {
        Error::DomainError(inner)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error::DomainError(DomainError::InvalidID) => {
                (StatusCode::NOT_FOUND, "User not found")
            }
            Error::AppError(AppError::Base) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Invalid username")
            }
            Error::AdopterError(AdapterError::Base) => {
                (StatusCode::NOT_FOUND, "User not found")
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}