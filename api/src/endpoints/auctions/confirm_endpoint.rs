use crate::di::AppState;
use application::use_cases::auctions::confirm_auction_use_case::AuctionConfirmationRequest;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::{Extension, Json};
use domain::entities::user::User;

pub async fn handle(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Path(auction_id): Path<String>,
    Json(request): Json<AuctionConfirmationRequest>,
) -> Result<impl axum::response::IntoResponse, domain::app_error::AppError> {
    state
        .modules
        .confirm_auction_use_case
        .execute(current_user, auction_id, request)
        .await
        .map(|_| axum::http::StatusCode::OK.into_response())
}
