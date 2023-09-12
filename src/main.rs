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

    #[arg(short = 'v', long = "verbose", default_value_t = false)]
    verbose: bool,

    #[arg(short = 'V', long = "veryverbose", default_value_t = false)]
    veryverbose: bool,
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

// Figure out how to pass state/values into a handler
async fn handler(test: String, ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
    println!("Received a new connection from {}", addr.ip());
    println!("Test Value Goes Here : {}", test);
    format!("{}\r\n", addr.ip())
}
