use axum::{
    extract::State,
    extract::TypedHeader,
    headers::UserAgent,
    routing::get, Router,
    http::Request,
    body::Body,
};
use axum_client_ip::XForwardedFor;
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

async fn handler(XForwardedFor(x_forwarded_for_ip): XForwardedFor, TypedHeader(user_agent): TypedHeader<UserAgent>, State(logging_state): State<AppState>, request: Request<Body>) -> String {
    if logging_state.verbosity == 1 {
        println!("Received a new connection from {:?}", x_forwarded_for_ip.first().unwrap());
        println!("UserAgent: {}", user_agent);
    }
    else if logging_state.verbosity == 2 {
        println!("Received a new connection from {:?}", x_forwarded_for_ip.first().unwrap());
        println!("Full Request Details : {:?}", request);
    }
    else {
        println!("Received a new connection from {:?}", x_forwarded_for_ip.first().unwrap());
    };

    format!("{:?}\r\n", x_forwarded_for_ip.first().unwrap())
}