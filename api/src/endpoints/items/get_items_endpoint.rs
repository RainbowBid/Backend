use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::Extension;

use crate::di::AppState;
use crate::endpoints::QueryFilterParamDto;
use domain::app_error::AppError;
use domain::entities::item::Category;
use domain::entities::user::User;

pub async fn handle(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Query(params): Query<QueryFilterParamDto>,
) -> Result<impl IntoResponse, AppError> {
    let category = params.category.map(Category::from);
    let response = state
        .modules
        .get_items_use_case
        .execute(user.id.to_string(), category)
        .await;
    Ok(response)
}
