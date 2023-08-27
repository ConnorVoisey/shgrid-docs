use axum::{
    extract::FromRef,
    routing::{delete, get, patch},
    Router,
};
use tower_http::cors::CorsLayer;
use sqlx::Pool;

use crate::models::contact::index_contact;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub database_pool: Pool<sqlx::Postgres>,
}

pub fn create_routes(database_pool: Pool<sqlx::Postgres>) -> Router {
    let app_state = AppState { database_pool };
    Router::new()
        .route("/", get(|| async { "Hello, Edited World!" }))
        .route("/contact", get(index_contact))
        .layer(CorsLayer::permissive())
        .with_state(app_state)
}
