use crate::di::AppState;
use crate::endpoints::QueryFilterParamDto;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use domain::app_error::AppError;
use domain::entities::item::Category;

use tracing::error;

pub async fn handle(
    State(state): State<AppState>,
    Query(params): Query<QueryFilterParamDto>,
) -> Result<impl IntoResponse, AppError> {
    let category = params.category.map(Category::from);

    state
        .modules
        .get_auctions_use_case
        .execute(category)
        .await
        .map_err(|e| {
            error!("Failed to get auctions: {:?}", e);
            e
        })
}
