use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use axum_valid::Valid;
use application::use_cases::get_items_use_case::dtos::GetAllItemsByUserIdRequest;
use domain::app_error::AppError;
use crate::di::AppState;

pub async fn handle(
    State(state): State<AppState>,
    Valid(Json(request)): Valid<Json<GetAllItemsByUserIdRequest>>,
) -> Result <impl IntoResponse, AppError>{
    let response = state
        .modules
        .get_items_use_case
        .execute(request.user_id)
        .await;
    Ok(response)
}
