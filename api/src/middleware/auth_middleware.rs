use crate::di::AppState;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::IntoResponse;
use domain::app_error::AppError;
use domain::app_error::AppError::InvalidJwt;
use domain::entities::token_claims::TokenClaims;
use domain::entities::user::User;
use jsonwebtoken::decode;
use tracing::{error, info};

pub async fn auth(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, AppError> {
    info!("Validating JWT");

    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| {
            if header.starts_with("Bearer ") {
                header.strip_prefix("Bearer ")
            } else {
                None
            }
        });

    let auth_header = match auth_header {
        Some(header) => header,
        None => return Err(InvalidJwt()),
    };

    match authorize_current_user(auth_header, &state).await {
        Ok(current_user) => {
            req.extensions_mut().insert(current_user);
            Ok(next.run(req).await)
        }
        Err(err) => {
            error!("Error authorizing user: {:?}", err);
            Err(err)
        }
    }
}

async fn authorize_current_user(auth_token: &str, state: &AppState) -> Result<User, AppError> {
    let claims = decode::<TokenClaims>(
        auth_token,
        &jsonwebtoken::DecodingKey::from_secret(state.config.jwt_key.as_ref()),
        &jsonwebtoken::Validation::default(),
    );

    match claims {
        Ok(claims) => {
            let user_id = claims.claims.sub;
            let user = state.modules.get_user_use_case.execute(user_id).await?;

            info!("User authorized: {}", user.name);
            Ok(user)
        }
        Err(err) => {
            error!("Error decoding token: {:?}", err);
            Err(InvalidJwt())
        }
    }
}
