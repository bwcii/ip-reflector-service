use axum::{
    extract::ConnectInfo, 
    routing::get, Router,
};
use std::net::{SocketAddr, Ipv4Addr};
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = String::from("0.0.0.0"))]
    listening_ip: String,

    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

#[tokio::main]
async fn main() {

    let args = Args::parse();
    let listening_ip_addr: Ipv4Addr = args.listening_ip.parse().expect("Unable to parse IP address");

    println!("Listening IP : {}", args.listening_ip);
    if args.listening_ip == String::from("0.0.0.0") {
        println!("CAUTION: Using 0.0.0.0 for the listening IP address effectively listens on ALL IP addresses.");
    }
    println!("Listening Port : {}", args.port);

    let app = Router::new()
    .route("/", get(handler));
    let addr = SocketAddr::from((listening_ip_addr, args.port));

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap()
}

async fn handler(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
    format!("{}\r\n", addr.ip())
}
