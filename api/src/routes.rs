use crate::di::Modules;
use crate::endpoints;
use axum::routing::post;
use axum::{Extension, Router};
use sqlx::PgPool;
use std::sync::Arc;

pub fn init_router(db: PgPool) -> Router {
    let modules = Arc::new(Modules::new(db));

    let auth_router = Router::new().route("/register", post(endpoints::auth::register::handle));

    let router = Router::new()
        .nest("/auth", auth_router)
        .layer(Extension(modules));

    router
}
