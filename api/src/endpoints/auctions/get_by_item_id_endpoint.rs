use crate::di::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Extension;
use domain::app_error::AppError;
use domain::entities::user::User;
use serde::Deserialize;
use tracing::error;

#[derive(Deserialize)]
pub struct PathParams {
    item_id: Option<String>,
}

pub async fn handle(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Path(path_params): Path<PathParams>,
) -> Result<impl IntoResponse, AppError> {
    let item_id = path_params.item_id.unwrap_or_else(|| "".to_string());
    if item_id.is_empty() {
        AppError::CannotGetAuctionForEmptyItemId();
    }

    let response = state
        .modules
        .get_by_item_id
        .execute(item_id)
        .await
        .map_err(|e| {
            error!("Failed to get auction: {:?}", e);
            e
        });
    response
}
