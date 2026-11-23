use actix_web_lab_derive::FromRequest;

#[derive(Debug, FromRequest)]
struct CopyAppData<T> {
    #[from_request(copy_from_app_data)]
    data: T,
}

fn assert_from_request<E: actix_web::FromRequest>() {}

fn main() {
    assert_from_request::<CopyAppData<u64>>();
}
