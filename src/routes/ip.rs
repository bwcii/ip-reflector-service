//use axum::http::HeaderMap;
use axum::http::HeaderMap;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    ip_address: String,
}


pub async fn ip(headers: HeaderMap) -> HeaderMap {

    //let ipaddr = headers.get("X-Forwarded-For").unwrap();

    //let data: Data = Data {
    //    ip_address: ipaddr.to_str().unwrap().to_string(),
    //};
    let data = headers;
    data
}