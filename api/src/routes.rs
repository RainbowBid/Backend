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
use tower_http::cors::CorsLayer;

pub fn init_router(db: PgPool, secrets: SecretStore) -> Router {
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
