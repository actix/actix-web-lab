use actix_web::web;
use actix_web_lab_derive::FromRequest;

#[derive(Debug, FromRequest)]
struct RequestParts<T> {
    pool: web::Data<T>,
}

fn assert_from_request<E: actix_web::FromRequest>() {}

fn main() {
    assert_from_request::<RequestParts<u32>>();
}
