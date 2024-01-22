use axum::http::HeaderMap;
use handlebars::{
    Handlebars,
    DirectorySourceOptions,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ClientIP {
    public_ip: String,
}

pub async fn handler(headers: HeaderMap) -> String {
//pub async fn handler() -> String {
    let mut handlebars = Handlebars::new();
    let directory_options = DirectorySourceOptions{tpl_extension: ".hbs".to_string(), hidden: false, temporary: false};
    
    handlebars.register_templates_directory("templates", directory_options).unwrap();

    let message = headers.get("X-Forwarded-For").unwrap();
    let message_string = message.to_str().unwrap().to_owned();
    let client_ip = ClientIP{public_ip: message_string};
    println!("{}", client_ip.public_ip);
    //"current IP: ".to_owned() + &message_string

    let html = handlebars.render("index", &client_ip).unwrap();
    html
}