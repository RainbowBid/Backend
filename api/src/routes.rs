use crate::di::AppState;
use crate::endpoints;
use axum::routing::post;
use axum::Router;
use sqlx::PgPool;

pub fn init_router(db: PgPool) -> Router {
    let app_state = AppState::new(db);

    let auth_router = Router::new().route(
        "/register",
        post(endpoints::auth::register_endpoint::handle),
    );

    let router = Router::new()
        .nest("/auth", auth_router)
        .with_state(app_state);

    router
}
