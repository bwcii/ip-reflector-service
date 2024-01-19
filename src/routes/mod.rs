mod index;
mod ip;

use axum::{Router, routing::get};

pub fn create_routes() -> Router {
    Router::new().route("/", get(index::handler))
    .route("/v1/ip", get(ip::ip))
}