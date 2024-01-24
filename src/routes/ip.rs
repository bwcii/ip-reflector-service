use axum::{http::HeaderMap, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct IPData {
    ip_address: String,
}


pub async fn ip(headers: HeaderMap) -> Json<IPData> {

    let ipaddr = headers.get("X-Forwarded-For").unwrap();

    let data: IPData = IPData {
        ip_address: ipaddr.to_str().unwrap().to_string(),
    };
    Json(data)
}