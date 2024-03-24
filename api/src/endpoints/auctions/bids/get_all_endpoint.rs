use crate::di::AppState;
use axum::extract::{Path,  State};
use axum::response::IntoResponse;
use domain::app_error::AppError;

use tracing::error;

pub async fn handle(
    State(state): State<AppState>,
    Path(auction_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    state
        .modules
        .get_bids_use_case
        .execute(auction_id)
        .await
        .map_err(|e| {
            error!("Failed to get auctions: {:?}", e);
            e
        })
}
