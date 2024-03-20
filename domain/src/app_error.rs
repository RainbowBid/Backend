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
    UserRegistrationFailed(#[source] anyhow::Error),
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
    #[error("Failed to create a new items")]
    CreateItemFailed(#[source] anyhow::Error),
    #[error("Invalid request: {0}")]
    InvalidRequest(#[from] validator::ValidationErrors),
    #[error("Failed to get image for items with id {0}")]
    GetItemImageFailed(#[source] anyhow::Error),
    #[error("Failed to get item with id {0}")]
    GetItemFailed(#[source] anyhow::Error),
    #[error("Item {0} does not belong to user {1}")]
    ItemDoesNotBelongToUser(String, String),
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
            AppError::InternalServerError() => {
                (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
            }
            AppError::CreateItemFailed(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
            }
            AppError::InvalidRequest(_) => (StatusCode::BAD_REQUEST, error_message).into_response(),
            AppError::GetItemImageFailed(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
            }
            AppError::GetItemFailed(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
            }
            AppError::ItemDoesNotBelongToUser(_, _) => {
                (StatusCode::FORBIDDEN, error_message).into_response()
            }
        }
    }
}
