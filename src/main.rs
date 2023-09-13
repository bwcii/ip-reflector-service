use axum::{
    extract::ConnectInfo, 
    extract::State,
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

#[derive(Clone)]
struct AppState {
    verbosity: u16,
}

#[tokio::main]
async fn main() {

    let args = Args::parse();
    let listening_ip_addr: Ipv4Addr = args.listening_ip.parse().expect("Unable to parse IP address");

    let mut logging_state: AppState = AppState {
        verbosity : 0,
    };

    if args.veryverbose {
        logging_state.verbosity = 2;
    }
    else if args.verbose {
        logging_state.verbosity = 1;
    };

    println!("Listening IP : {}", args.listening_ip);
    if args.listening_ip == String::from("0.0.0.0") {
        println!("CAUTION: Using 0.0.0.0 for the listening IP address effectively listens on ALL IP addresses.");
    }
    println!("Listening Port : {}", args.port);

    let app = Router::new()
    .route("/", get(handler))
    .with_state(logging_state);

    let addr = SocketAddr::from((listening_ip_addr, args.port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap()
}

async fn handler(State(logging_state): State<AppState>, ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
    if logging_state.verbosity == 1 {
        println!("Received a new connection from {}", addr.ip());
    }
    else if logging_state.verbosity == 2 {
        println!("Very Verbose Log goes here!");
    }
    else {
        println!("A new connection was made!");
    };

    format!("{}\r\n", addr.ip())
}
