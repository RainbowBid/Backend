use crate::di::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Extension;
use domain::app_error::AppError;
use domain::entities::user::User;

pub async fn handle(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let response = state.modules.get_item_use_case.execute(user, id).await;
    Ok(response)
}
