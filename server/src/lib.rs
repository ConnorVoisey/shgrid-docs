pub mod config;
pub mod error;
pub mod models;
pub mod query_params;
pub mod routes;

use config::Config;
use routes::create_routes;
use std::{net::SocketAddr, sync::Arc};
use tracing::info;

pub async fn run() {
    let config = Arc::new(Config::init());
    let app = create_routes(config.clone());
    let port = config.port;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    // serve the api
    info!("hosting server at 127.0.0.1:{port}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
