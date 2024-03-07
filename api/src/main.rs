mod di;
mod endpoints;
mod routes;

use crate::routes::init_router;
// shuttle_secrets::{SecretStore};
use sqlx::PgPool;
use tracing::info;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] db: PgPool,
    //#[shuttle_secrets::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    info!("Starting server...");
    // let allowed_origin = secrets
    //     .get("ALLOWED_ORIGIN")
    //     .expect("You need to set your ALLOWED_ORIGIN secret!");
    Ok(init_router(db).into())
}
