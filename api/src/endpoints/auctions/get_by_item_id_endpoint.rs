use crate::di::AppState;
use application::use_cases::auctions::create_auction_use_case::dtos::CreateAuctionRequest;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::{Extension, Json};
use axum_valid::Valid;
use domain::app_error::AppError;
use domain::entities::item::Category;
use domain::entities::user::User;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

#[derive(Deserialize)]
pub struct PathParams {
    itemId: Option<String>,
}
pub async fn handle(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Path(path_params): Path<PathParams>,
) -> Result<impl IntoResponse, AppError> {

    let item_id = match path_params.itemId {
        Some(item_id) => item_id,
        None => "".to_string(),
    };
    info!("Getting1 auction with item_id: {}", item_id);
    if item_id == "" {
        AppError::CannotGetAuctionForEmptyItemId();
    }
    info!("Getting2 auction with item_id: {}", item_id);

    let response = state
        .modules
        .get_by_item_id
        .execute(item_id)
        .await
        .map(|result| result)
        .map_err(|e| {
            error!("Failed to get auction: {:?}", e);
            e
        });
    response
}
