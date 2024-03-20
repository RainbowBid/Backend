use crate::di::AppState;
use application::use_cases::auctions::create_auction_use_case::dtos::CreateAuctionRequest;
use axum::extract::State;
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
    Valid(Json(request)): Valid<Json<CreateAuctionRequest>>,
) -> Result<impl IntoResponse, AppError> {
    let response = state
        .modules
        .create_auction_use_case
        .execute(current_user, request)
        .await
        .map(|_| StatusCode::CREATED.into_response())
        .map_err(|e| {
            error!("Failed to create auction: {:?}", e);
            e
        })?;

    Ok(response)
}
