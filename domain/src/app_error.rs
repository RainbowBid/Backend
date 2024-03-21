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
    #[error("Failed to create auction: {0}")]
    CreateAuctionFailed(#[source] anyhow::Error),
    #[error("Cannot create auction for item with id: {1} that does not belong to current user with id: {0}")]
    CannotCreateAuctionForItemThatDoesNotBelongToCurrentUser(String, String),
    #[error("Cannot create auction for non-existing item with id: {0}")]
    CannotCreateAuctionForNonExistingItem(String),
    #[error("Cannot create auction for item with id {0} that already has an ongoing auction")]
    CannotCreateAuctionForItemWithOngoingAuction(String),
    #[error("Cannot get auction for empty item_id")]
    CannotGetAuctionForEmptyItemId(),
    #[error("Failed to get auction for item_id {0}")]
    GetAuctionFailed(#[source] anyhow::Error),
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
            AppError::CreateAuctionFailed(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
            }
            AppError::CannotCreateAuctionForItemThatDoesNotBelongToCurrentUser(_, _) => {
                (StatusCode::FORBIDDEN, error_message).into_response()
            }
            AppError::CannotCreateAuctionForNonExistingItem(_) => {
                (StatusCode::FORBIDDEN, error_message).into_response()
            }
            AppError::CannotCreateAuctionForItemWithOngoingAuction(_) => {
                (StatusCode::FORBIDDEN, error_message).into_response()
            }
            AppError::CannotGetAuctionForEmptyItemId() => {
                (StatusCode::FORBIDDEN, error_message).into_response()

            }
            AppError::GetAuctionFailed(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()

            }
        }
    }
}
