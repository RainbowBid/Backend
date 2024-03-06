use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use serde_json::json;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Username {0} already exists")]
    UsernameAlreadyExists(String),
    #[error("Email {0} already exists")]
    EmailAlreadyExists(String),
    #[error("User registration failed")]
    UserRegistrationFailed(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let error_message = json!({"error": self.to_string()}).to_string();

        match self {
            AppError::UsernameAlreadyExists(_) => {
                (StatusCode::CONFLICT, error_message).into_response()
            }
            AppError::EmailAlreadyExists(_) => {
                (StatusCode::CONFLICT, error_message).into_response()
            }
            AppError::UserRegistrationFailed(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
            }
        }
    }
}
