
use crate::di::AppState;
use crate::endpoints;
use axum::http::header::{ACCEPT, AUTHORIZATION, ORIGIN};
use axum::http::{HeaderValue, Method};
use axum::routing::post;
use axum::Router;
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

pub fn init_router(db: PgPool) -> Router {
    let  allowed_url = "http://localhost:10000";
    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec![ORIGIN, AUTHORIZATION, ACCEPT])
        .allow_origin(allowed_url.parse::<HeaderValue>().unwrap());

    let app_state = AppState::new(db);

    let auth_router = Router::new().route(
        "/register",
        post(endpoints::auth::register_endpoint::handle),
    );

    let router = Router::new()
        .nest("/auth", auth_router)
        .with_state(app_state)
        .layer(cors);

    router
}
