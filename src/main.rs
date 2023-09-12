use axum::{
    extract::ConnectInfo, 
    routing::get, Router,
    http::Request,
    body::Body,
};
use std::net::SocketAddr;


#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(handler))
    .route("/test", get(request));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap()
}

async fn handler(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
    format!("{}\r\n", addr.ip())
}

async fn request(request: Request<Body>) {
    println!("{:?}", request);
}
