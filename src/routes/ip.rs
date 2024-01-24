use axum::{http::HeaderMap, Json};
use serde::Serialize;
//use std::env;

#[derive(Serialize)]
pub struct IPData {
    ip_address: String,
}

#[derive(Serialize)]
pub struct IPDataGeo {
        ip: String,
        //hostname: String,
        //continent_code: String,
        //continent_name: String,
        //country_code2: String,
        //country_code3: String,
        //country_name: String,
        //country_capital: String,
        //state_prov: String,
        //district: String,
        //city: String,
        //zipcode: String,
        //latitude: String,
        //longitude: String,
        //is_eu: bool,
        //calling_code: String,
        //country_tld: String,
        //languages: String,
        //country_flag: String,
        //geoname_id: String,
        //isp: String,
        //connection_type: String,
        //organization: String,
        //asn: String,
}


pub async fn ip(headers: HeaderMap) -> Json<IPData> {

    let ipaddr = headers.get("X-Forwarded-For").unwrap();

    let data: IPData = IPData {
        ip_address: ipaddr.to_str().unwrap().to_string(),
    };
    Json(data)
}

//pub async fn ip_geo(headers: HeaderMap) -> Json<IPDataGeo> {
//    let apikeytest = env::var("IPGEOAPIKEY").unwrap();
//    let ipaddr = headers.get("X-Forwarded-For").unwrap();
//    let data: IPDataGeo = IPDataGeo {
//        ip: ipaddr.to_str().unwrap().to_string(),
//    };
//    Json(data)
//}