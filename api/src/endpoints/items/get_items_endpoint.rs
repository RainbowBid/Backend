use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Extension;
use serde::Deserialize;

use crate::di::AppState;
use domain::app_error::AppError;
use domain::entities::item::Category;
use domain::entities::user::User;

#[derive(Deserialize)]
pub struct QueryParamsDto {
    category: Option<String>,
}

pub async fn handle(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Query(params): Query<QueryParamsDto>,
) -> Result<impl IntoResponse, AppError> {
    let category = match params.category {
        Some(category) => Some(Category::from(category)),
        None => None,
    };
    let response = state
        .modules
        .get_items_use_case
        .execute(user.id.to_string(), category)
        .await;
    Ok(response)
}
