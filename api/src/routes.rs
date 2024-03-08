use crate::di::AppState;
use crate::endpoints;
use axum::http::header::{
    ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_REQUEST_HEADERS,
    ACCESS_CONTROL_REQUEST_METHOD, AUTHORIZATION, CONTENT_TYPE, ORIGIN,
};
use axum::http::{HeaderValue, Method};
use axum::routing::post;
use axum::Router;
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

pub fn init_router(db: PgPool, allowed_origin: String) -> Router {
    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(vec![
            ORIGIN,
            AUTHORIZATION,
            ACCEPT,
            ACCESS_CONTROL_REQUEST_HEADERS,
            ACCESS_CONTROL_REQUEST_METHOD,
            CONTENT_TYPE,
            ACCESS_CONTROL_ALLOW_HEADERS,
        ])
        .allow_origin(allowed_origin.parse::<HeaderValue>().unwrap());

    let app_state = AppState::new(db);

    let auth_router = Router::new().route(
        "/register",
        post(endpoints::auth::register_endpoint::handle),
    ).route("/login", post(endpoints::auth::login_endpoint::handle));

    let router = Router::new()
        .nest("/auth", auth_router)
        .with_state(app_state)
        .layer(cors);

    router
}
