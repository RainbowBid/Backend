use crate::di::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use domain::app_error::AppError;
use tracing::error;

pub async fn handle(
    State(state): State<AppState>,
    Path(item_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    state
        .modules
        .get_by_item_id
        .execute(item_id)
        .await
        .map_err(|e| {
            error!("Failed to get auction: {:?}", e);
            e
        })
}
