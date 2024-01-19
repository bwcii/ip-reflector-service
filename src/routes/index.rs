use axum::http::HeaderMap;

pub async fn handler(headers: HeaderMap) -> String {
    let message = headers.get("forwarded").unwrap();
    let message_string = message.to_str().unwrap().to_owned();
    message_string
}