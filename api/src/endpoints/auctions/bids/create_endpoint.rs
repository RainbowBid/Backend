use crate::di::AppState;
use application::use_cases::auctions::create_auction_use_case::dtos::CreateAuctionRequest;
use application::use_cases::bids::create_bid_use_case::dtos::CreateBidRequest;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::{Extension, Json};
use axum_valid::Valid;
use domain::app_error::AppError;
use domain::entities::user::User;
use http::StatusCode;
use tracing::error;

pub async fn handle(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Path(auction_id): Path<String>,
    Valid(Json(request)): Valid<Json<CreateBidRequest>>,
) -> Result<impl IntoResponse, AppError> {

    let request = CreateBidRequest {
        auction_id: auction_id.clone(),
        ..request
    };

    let response = state
        .modules
        .create_bid_use_case
        .execute(current_user, request)
        .await
        .map(|_| StatusCode::CREATED.into_response())
        .map_err(|e| {
            error!("Failed to create bid for {:?}", e);
            e
        })?;

    Ok(response)
}
