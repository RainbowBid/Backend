use crate::di::AppState;
use crate::endpoints;
use crate::middleware::auth_middleware::auth;
use axum::http::header::{
    ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_REQUEST_HEADERS,
    ACCESS_CONTROL_REQUEST_METHOD, AUTHORIZATION, CONTENT_TYPE, ORIGIN,
};
use axum::http::{HeaderValue, Method};
use axum::routing::{get, post};
use axum::{middleware, Router};
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};
use tower_http::cors::CorsLayer;
use tracing::log::{error, info};

pub async fn init_router(db: PgPool, secrets: SecretStore) -> Router {
    let app_state = AppState::new(db, secrets);

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
        .expose_headers(vec![
            ORIGIN,
            AUTHORIZATION,
            ACCEPT,
            ACCESS_CONTROL_REQUEST_HEADERS,
            ACCESS_CONTROL_REQUEST_METHOD,
            CONTENT_TYPE,
            ACCESS_CONTROL_ALLOW_HEADERS,
        ])
        .allow_origin(
            app_state
                .config
                .allowed_origin
                .parse::<HeaderValue>()
                .unwrap(),
        );

    match init_job_scheduler(app_state.clone()).await {
        Ok(_) => info!("Job scheduler started."),
        Err(e) => error!("Error. Job scheduler failed to start: {:?}", e),
    }

    let auth_router = Router::new()
        .route(
            "/register",
            post(endpoints::auth::register_endpoint::handle),
        )
        .route("/login", post(endpoints::auth::login_endpoint::handle));

    let item_router = Router::new()
        .route(
            "/create",
            post(endpoints::items::create_endpoint::handle)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route(
            "/:id/image",
            get(endpoints::items::get_image_endpoint::handle)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route(
            "/:id",
            get(endpoints::items::get_item_endpoint::handle)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route(
            "/all",
            get(endpoints::items::get_items_endpoint::handle)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        );

    let auction_router = Router::new()
        .route(
            "/create",
            post(endpoints::auctions::create_endpoint::handle)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route(
            "/:item_id",
            get(endpoints::auctions::get_by_item_id_endpoint::handle),
        )
        .route("/all", get(endpoints::auctions::get_all_endpoint::handle))
        .route(
            "/:auction_id/bids/create",
            post(endpoints::auctions::bids::create_endpoint::handle)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        )
        .route(
            "/:auction_id/bids/all",
            get(endpoints::auctions::bids::get_all_endpoint::handle)
                .route_layer(middleware::from_fn_with_state(app_state.clone(), auth)),
        );

    Router::new()
        .nest("/auth", auth_router)
        .nest("/items", item_router)
        .nest("/auctions", auction_router)
        .with_state(app_state)
        .layer(cors)
}

// pub async fn init_job_scheduler(app_state: AppState) -> Result<(), JobSchedulerError> {
//     let scheduler = JobScheduler::new().await?;
//     let app_state_clone = Arc::new(app_state.clone());
//     scheduler.add(
//         Job::new_async(app_state.config.clone().finalize_auctions_cron.clone().as_str(), |uuid, mut l| {
//             Box::pin(async move {
//                 info!("Handle expired auctions job runs.");
//
//                 match &app_state_clone.modules.clone().handle_expired_auctions_use_case.execute().await{
//                     Ok(_) => info!("Expired auctions job succeeded."),
//                     Err(e) => error!("Error. Expired auctions job failed: {:?}", e),
//                 }
//             })
//         })?
//     ).await?;
//
//     scheduler.start().await?;
//     Ok(())
// }
pub async fn init_job_scheduler(app_state: AppState) -> Result<(), JobSchedulerError> {
    let scheduler = JobScheduler::new().await?;
    let app_state_clone = Arc::new(app_state.clone());
    let app_state_clone_for_closure = app_state_clone.clone(); // Clone app_state_clone here
    scheduler
        .add(Job::new_async(
            app_state
                .config
                .clone()
                .finalize_auctions_cron
                .clone()
                .as_str(),
            move |_, _| {
                let app_state_clone = app_state_clone_for_closure.clone(); // Use the clone inside the closure
                Box::pin(async move {
                    info!("Handle expired auctions job runs.");

                    match app_state_clone
                        .modules
                        .handle_expired_auctions_use_case
                        .execute()
                        .await
                    {
                        Ok(_) => info!("Expired auctions job succeeded."),
                        Err(e) => error!("Error. Expired auctions job failed: {:?}", e),
                    }
                })
            },
        )?)
        .await?;

    scheduler.start().await?;
    Ok(())
}
