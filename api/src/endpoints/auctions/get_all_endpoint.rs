use crate::di::AppState;
use crate::endpoints::auctions::get_by_item_id_endpoint::PathParams;
use crate::endpoints::QueryFilterParamDto;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Extension;
use domain::app_error::AppError;
use domain::entities::auction::AuctionWithItem;
use domain::entities::item::Category;
use domain::entities::user::User;
use serde::{Deserialize, Serialize};
use tracing::error;

pub async fn handle(
    State(state): State<AppState>,
    Query(params): Query<QueryFilterParamDto>,
) -> Result<impl IntoResponse, AppError> {
    let category = match params.category {
        Some(category) => Some(Category::from(category)),
        None => None,
    };

    let response = state
        .modules
        .get_auctions_use_case
        .execute(category)
        .await
        .map_err(|e| {
            error!("Failed to get auctions: {:?}", e);
            e
        });
    response
}
