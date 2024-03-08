use crate::di::AppState;
use application::use_cases::login_use_case::dtos::LoginRequest;
use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum_valid::Valid;
use domain::app_error::AppError;
use domain::entities::token_claims::TokenClaims;
use http::HeaderValue;
use jsonwebtoken::{encode, EncodingKey, Header};
use shuttle_runtime::__internals::serde_json::json;
use chrono::{Duration,Utc};

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
            .unwrap();

            let mut response = Response::new(json!({}).to_string());
            response
                .headers_mut()
                .insert("Jwt", HeaderValue::from_str(&token).unwrap())
                .expect("Can't build jwt");
            response
        });
    response
}
