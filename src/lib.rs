mod routes;

use routes::create_routes;

pub async fn run() {
    let app = create_routes();

    let binding_socket = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(binding_socket, app)
        .await
        .unwrap();
    
}