use actix_web::http;
use actix_web_lab_derive::FromRequest;

#[derive(Debug, FromRequest)]
struct RequestParts {
    #[allow(dead_code)]
    method: http::Method,
}

fn main() {}
