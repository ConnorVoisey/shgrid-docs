use axum::{
    extract::FromRef,
    routing::{delete, get, patch},
    Router,
};
use tower_http::cors::CorsLayer;
use sqlx::Pool;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub database_pool: Pool<sqlx::Postgres>,
}

pub fn create_routes(database_pool: Pool<sqlx::Postgres>) -> Router {
    let app_state = AppState { database_pool };
    Router::new()
        .route("/", get(|| async { "Hello, Edited World!" }))
        .layer(CorsLayer::permissive())
        .with_state(app_state)
}
