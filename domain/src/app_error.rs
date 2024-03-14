use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Username {0} already exists")]
    UsernameAlreadyExists(String),
    #[error("Email {0} already exists")]
    EmailAlreadyExists(String),
    #[error("User registration failed")]
    UserRegistrationFailed(#[from] anyhow::Error),
    #[error("Email {0} is not registered.")]
    NotRegisteredEmail(String),
    #[error("User login failed. Bad password.")]
    BadPassword(),
    #[error("Missing or expired jwt.")]
    InvalidJwt(),
    #[error("User with id {0} not found")]
    UserNotFound(String),
    #[error("Internal server error")]
    InternalServerError(),
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
            AppError::NotRegisteredEmail(_) => {
                (StatusCode::UNAUTHORIZED, error_message).into_response()
            }
            AppError::BadPassword() => (StatusCode::UNAUTHORIZED, error_message).into_response(),
            AppError::InvalidJwt() => (StatusCode::UNAUTHORIZED, error_message).into_response(),
            AppError::UserNotFound(_) => (StatusCode::NOT_FOUND, error_message).into_response(),
            AppError::InternalServerError() => (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response(),
        }
    }
}
