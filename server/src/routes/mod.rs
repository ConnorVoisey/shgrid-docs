use axum::{extract::FromRef, routing::get, Router};
use sqlx::Pool;
use tower_http::cors::CorsLayer;

use crate::models::contact::{index_contact, show_contact};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub database_pool: Pool<sqlx::Postgres>,
}

pub fn create_routes(database_pool: Pool<sqlx::Postgres>) -> Router {
    let app_state = AppState { database_pool };
    Router::new()
        .route("/", get(|| async { "Hello, Edited World!" }))
        .route("/contact", get(index_contact))
        .route("/contact/:id", get(show_contact))
        .layer(CorsLayer::permissive())
        .with_state(app_state)
}
