use actix_web_lab_derive::FromRequest;

#[derive(FromRequest)]
struct OnlyAppData {
    #[from_request(copy_from_app_data)]
    int: u64,
}

fn main() {}
