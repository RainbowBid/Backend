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
    GetAuctionFailed(String),
    #[error("No auction found for item_id {0}")]
    NoAuctionFoundForItemId(String),
    #[error("No auction found for id {0}")]
    NoAuctionFoundForId(String),
    #[error("Failed to get auctions.")]
    FailedToGetAuctions(),
    #[error("Failed to create bid.")]
    CreateBidFailed(#[source] anyhow::Error),
    #[error("Failed to create bid. Internal server error.")]
    CreateBidFailedInternalServerError(#[source] anyhow::Error),
    #[error("Owner cannot bid to its auction")]
    OwnerCannotBid(),
    #[error("Bid amount ({0}) must be greater than the current highest bid ({1}).")]
    BidAmountMustBeGreaterThanCurrentHighestBid(f32, f32),
    #[error("Bid amount ({0}) must be greater than the auction starting price ({1}).")]
    BidAmountMustBeGreaterThanStartingPrice(f32, f32),
    #[error("Cannot confirm auction if user is not the owner of the auction.")]
    CannotConfirmAuctionIfUserIsNotOwner(),
    #[error("Auction confirmation failed.")]
    AuctionConfirmationFailed(),
    #[error("Cannot confirm auction if strategy is not RequestFinalApproval")]
    CannotConfirmAuctionIfStrategyIsNotRequestFinalApproval(),
    #[error("Cannot confirm auction if auction is not expired")]
    CannotConfirmAuctionIfAuctionIsNotExpired(),
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
            AppError::NoAuctionFoundForItemId(_) => {
                (StatusCode::NOT_FOUND, error_message).into_response()
            }
            AppError::FailedToGetAuctions() => {
                (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
            }
            AppError::CreateBidFailed(_) => {
                (StatusCode::BAD_REQUEST, error_message).into_response()
            }
            AppError::CreateBidFailedInternalServerError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
            }
            AppError::OwnerCannotBid() => (StatusCode::FORBIDDEN, error_message).into_response(),
            AppError::BidAmountMustBeGreaterThanCurrentHighestBid(_, _) => {
                (StatusCode::BAD_REQUEST, error_message).into_response()
            }
            AppError::BidAmountMustBeGreaterThanStartingPrice(_, _) => {
                (StatusCode::BAD_REQUEST, error_message).into_response()
            }
            AppError::NoAuctionFoundForId(_) => {
                (StatusCode::NOT_FOUND, error_message).into_response()
            }
            AppError::CannotConfirmAuctionIfUserIsNotOwner() => {
                (StatusCode::FORBIDDEN, error_message).into_response()
            }
            AppError::AuctionConfirmationFailed() => {
                (StatusCode::INTERNAL_SERVER_ERROR, error_message).into_response()
            }
            AppError::CannotConfirmAuctionIfStrategyIsNotRequestFinalApproval() => {
                (StatusCode::FORBIDDEN, error_message).into_response()
            }
            AppError::CannotConfirmAuctionIfAuctionIsNotExpired() => {
                (StatusCode::FORBIDDEN, error_message).into_response()
            }
        }
    }
}
