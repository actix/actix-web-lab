use actix_web::http;
use actix_web_lab_derive::FromRequest;

#[derive(FromRequest)]
struct InvalidAttr {
    #[from_request(unknown_option)]
    method: http::Method,
}

fn main() {}
