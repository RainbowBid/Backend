use crate::di::AppState;
use application::use_cases::item::create_item_use_case::dtos::CreateItemRequest;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use axum_typed_multipart::TypedMultipart;
use domain::app_error::AppError;
use domain::entities::user::User;
use validator::Validate;

pub async fn handle(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    TypedMultipart(request): TypedMultipart<CreateItemRequest>,
) -> Result<impl IntoResponse, AppError> {
    match request.validate() {
        Ok(_) => state
            .modules
            .create_item_use_case
            .execute(current_user, request)
            .await
            .map(|_| StatusCode::CREATED.into_response()),
        Err(e) => Err(AppError::InvalidRequest(e)),
    }
}
