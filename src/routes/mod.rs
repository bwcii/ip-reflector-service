mod index;
mod two;

use axum::{Router, routing::get};

pub fn create_routes() -> Router {
    Router::new().route("/", get(index::handler))
    .route("/two", get(two::two))
}