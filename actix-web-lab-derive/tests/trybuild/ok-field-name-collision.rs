use actix_web::{http, web};
use actix_web_lab_derive::FromRequest;

#[derive(FromRequest)]
struct RequestParts {
    #[from_request(copy_from_app_data)]
    req: u64,
    #[from_request(copy_from_app_data)]
    pl: u32,
    #[from_request(copy_from_app_data)]
    method_fut: u16,
    method: http::Method,
    pool: web::Data<u32>,
}

fn main() {}
