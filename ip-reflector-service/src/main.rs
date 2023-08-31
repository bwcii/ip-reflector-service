use axum::{extract::ConnectInfo, routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Route all requests on "/" endpoint to anonymous handler.
    //
    // A handler is an async function which returns something that implements
    // `azum::response::IntoResponse`.

    // A closure or a function can be used as a handler.

    let app = Router::new().route("/", get(handler));
    //        Router::new().route("/", get(|| async { "Hello, world!" }));
    //app.into_make_service_with_connect_info()
    // Address that the server will bind to.
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    // Use `hyper::server::Server` which is re-exported through `axum::Server` to serve the app.
    axum::Server::bind(&addr)
        // Hyper server takes a make service.
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap()
}

//async fn handler() -> &'static str {
//    "Hello, world!"
//}
async fn handler(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
    format!("{}\r\n", addr.ip())
}
