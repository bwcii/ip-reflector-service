mod routes;

use routes::create_routes;
use std::env;

pub async fn run() {
    let listening_ip = "0.0.0.0";
    let listening_port = env::var("PORT").unwrap_or_else(|_default_port| {
        let listening_port = "8080".to_string();
        listening_port
    });
    let listening_socket = listening_ip.to_owned() + ":" + &listening_port;
    println!("Listening On : {}", listening_socket);
    let apikeytest = env::var("IPGEOAPIKEY").unwrap();
    println!("API Key : {}", apikeytest);

    let app = create_routes();

    let binding_socket = tokio::net::TcpListener::bind(listening_socket).await.unwrap();

    axum::serve(binding_socket, app)
        .await
        .unwrap();
    
}