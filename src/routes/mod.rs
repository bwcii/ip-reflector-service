mod index;
mod ip;
mod favicon;

use axum::{Router, routing::get, http::Method};
use tower_http::cors::{CorsLayer, Any};

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET,])
        .allow_origin(Any);

    Router::new().route("/", get(index::handler))
    .route("/v1/ip", get(ip::ip))
    .route("/favicon.ico", get(favicon::favicon))
    .layer(cors)
}