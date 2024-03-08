use crate::di::AppState;
use application::use_cases::login_use_case::dtos::LoginRequest;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum_valid::Valid;
use chrono::{Duration, Utc};
use domain::app_error::AppError;
use domain::entities::token_claims::TokenClaims;
use http::HeaderValue;
use jsonwebtoken::{encode, EncodingKey, Header};
use shuttle_runtime::__internals::serde_json::json;

pub async fn handle(
    State(state): State<AppState>,
    Valid(Json(request)): Valid<Json<LoginRequest>>,
) -> Result<impl IntoResponse, AppError> {
    let response = state
        .modules
        .login_use_case
        .execute(request)
        .await
        .map(|user| {
            let exp = (Utc::now() + Duration::minutes(60)).timestamp() as usize;
            let claims: TokenClaims = TokenClaims {
                username: user.id.to_string(),
                exp,
            };

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(state.config.jwt_key.as_ref()),
            )
            .map_err(|_| AppError::InvalidJwt())?;

            let mut response = Response::new(json!({}).to_string());
            response
                .headers_mut()
                .insert(
                    "Authorization",
                    HeaderValue::from_str(format!("Bearer {}", &token).as_str())
                        .map_err(|_| AppError::InvalidJwt())?,
                );

            Ok(response)
        })?;

    response
}
