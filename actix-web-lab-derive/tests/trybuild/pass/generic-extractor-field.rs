use actix_web::http::Method;
use actix_web_lab_derive::FromRequest;

#[derive(Debug, FromRequest)]
struct DirectExtractor<T> {
    value: T,
}

fn assert_from_request<E: actix_web::FromRequest>() {}

fn main() {
    assert_from_request::<DirectExtractor<Method>>();
}
