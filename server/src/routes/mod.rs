pub mod contact;
pub mod organisation;
use crate::routes::organisation::index_organisations;
use crate::{
    config::Config,
    models::{
        contact::{create_contacts, Contact},
        organisation::{create_organisations, Organisation},
    },
};
use axum::{extract::FromRef, routing::get, Router};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

use self::contact::index_contacts;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub config: Arc<Config>,
    pub organisations: Vec<Organisation>,
    pub contacts: Vec<Contact>,
}

pub fn create_routes(config: Arc<Config>) -> Router {
    let organisations = create_organisations();
    let app_state = Arc::new(AppState {
        config,
        contacts: create_contacts(&organisations),
        organisations,
    });
    Router::new()
        .route("/", get(|| async { "Hello, Edited World!" }))
        .route("/organisation", get(index_organisations))
        .route("/contact", get(index_contacts))
        // .route("/contact/:id", get(show_contact))
        .layer(CorsLayer::permissive())
        .with_state(app_state)
}
