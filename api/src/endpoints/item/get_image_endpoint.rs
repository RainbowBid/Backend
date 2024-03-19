use crate::di::AppState;
use axum::body::Bytes;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Extension;
use domain::app_error::AppError;
use domain::entities::user::User;

pub async fn handle(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Extension(current_user): Extension<User>,
) -> Result<impl IntoResponse, AppError> {
    let response = state
        .modules
        .get_item_image_use_case
        .execute(current_user, id)
        .await
        .map(|picture_data| {
            let picture_data: Bytes = picture_data.into();

            Ok(([("content-type", "image/png")], picture_data).into_response())
        })?;

    response
}
