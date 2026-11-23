use actix_web::{http::Method, web};
use actix_web_lab_derive::FromRequest;

#[derive(Debug, FromRequest)]
struct MultiGeneric<T: Clone, U, V>
where
    U: Default,
{
    pool: web::Data<T>,
    value: U,
    #[from_request(copy_from_app_data)]
    copied: V,
}

fn assert_from_request<E: actix_web::FromRequest>() {}

fn main() {
    assert_from_request::<MultiGeneric<String, Method, u64>>();
}
