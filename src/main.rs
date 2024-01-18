use axum::{routing::get, Router, http::HeaderMap};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let binding_socket = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();

    axum::serve(binding_socket, app)
        .await
        .unwrap();
    
}

async fn handler(headers: HeaderMap) -> &'static str {
    println!("{:?}", headers);
    "Hello World!"
}