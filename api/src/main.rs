use axum::{routing::get, Router};
use sqlx::PgPool;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] db: PgPool) -> shuttle_axum::ShuttleAxum {
    // sqlx::migrate!().run(&db).await.expect("Migrations failed");

    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}
