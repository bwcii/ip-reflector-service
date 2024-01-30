use axum::{
    http::HeaderMap, 
    response::IntoResponse,
    Json};
use serde::{Deserialize, Serialize};
use reqwest;
use std::env;

#[derive(Serialize, Deserialize)]
pub struct IPData {
    ip_address: String,
}

// ipgeolocation.io response structs
// Currency struct
#[derive(Serialize, Deserialize)]
pub struct Currency {
    code: String,
    name: String,
    symbol: String,
}

// Time_Zone struct
#[derive(Serialize, Deserialize)]
pub struct TimeZone {
    name: String,
    offset: i8,
    current_time: String,
    current_time_unix: f32,
    is_dst: bool,
    dst_savings: i8,
}

#[derive(Serialize, Deserialize)]
pub struct IPDataGeo {
        ip: String,
        continent_code: String,
        continent_name: String,
        country_code2: String,
        country_code3: String,
        country_name: String,
        country_capital: String,
        state_prov: String,
        district: String,
        city: String,
        zipcode: String,
        latitude: String,
        longitude: String,
        is_eu: bool,
        calling_code: String,
        country_tld: String,
        languages: String,
        country_flag: String,
        geoname_id: String,
        isp: String,
        connection_type: String,
        organization: String,
        currency: Currency,
        time_zone: TimeZone,
}


pub async fn ip(headers: HeaderMap) -> Json<IPData> {

    let ipaddr = headers.get("X-Forwarded-For").unwrap();

    let data: IPData = IPData {
        ip_address: ipaddr.to_str().unwrap().to_string(),
    };
    Json(data)
}

pub async fn ip_geo(headers: HeaderMap) -> impl IntoResponse {
    let ip_geo_apikey = env::var("IPGEOAPIKEY").unwrap();
    let ipaddr = headers.get("X-Forwarded-For").unwrap();
    let url = format!("https://api.ipgeolocation.io/ipgeo?apiKey={}&ip={}", ip_geo_apikey, ipaddr.to_str().unwrap());

    let body = reqwest::get(url).await.unwrap().json::<IPDataGeo>().await.unwrap();
    Json(body)
}