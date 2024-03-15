use axum::extract::State;
use axum::response::IntoResponse;
use axum::Extension;

use crate::di::AppState;
use domain::app_error::AppError;
use domain::entities::user::User;

pub async fn handle(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, AppError> {
    let response = state
        .modules
        .get_items_use_case
        .execute(user.id.to_string())
        .await;
    Ok(response)
}
