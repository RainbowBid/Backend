mod di;
mod endpoints;
mod routes;

use crate::routes::init_router;
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use tracing::info;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] db: PgPool,
    #[shuttle_secrets::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    info!("Starting server...");
    Ok(init_router(db, secrets).into())
}
