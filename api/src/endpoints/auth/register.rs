use crate::di::Modules;
use application::use_cases::register_use_case::RegisterRequest;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use axum_valid::Valid;
use std::sync::Arc;

pub async fn handle(
    Extension(modules): Extension<Arc<Modules>>,
    Valid(Json(request)): Valid<Json<RegisterRequest>>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let use_case = &modules.register_use_case;

    use_case
        .execute(request)
        .await
        .map(|_| StatusCode::CREATED)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
