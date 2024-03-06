use crate::di::AppState;
use application::use_cases::register_use_case::RegisterRequest;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum_valid::Valid;
use domain::app_error::AppError;

pub async fn handle(
    State(state): State<AppState>,
    Valid(Json(request)): Valid<Json<RegisterRequest>>,
) -> Result<impl IntoResponse, AppError> {
    state
        .modules
        .register_use_case
        .execute(request)
        .await
        .map(|_| StatusCode::CREATED.into_response())
}
