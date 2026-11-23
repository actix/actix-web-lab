#![allow(missing_docs)]

use actix_web::{
    App, HttpResponse, Responder,
    http::{Method, StatusCode},
    web,
};
use actix_web_lab_derive::FromRequest;

#[derive(Debug, FromRequest)]
struct RequestParts {
    method: Method,
    pool: web::Data<u32>,
    body: String,
    body2: String,

    #[from_request(copy_from_app_data)]
    copied_data: u64,
}

async fn handler(parts: RequestParts) -> impl Responder {
    let RequestParts {
        method,
        pool,
        body,
        body2,
        copied_data,
        ..
    } = parts;

    let pool = **pool;

    assert_eq!(copied_data, 43);

    assert_eq!(body, "foo");

    // assert that body is taken and second attempt to do so will be blank
    assert_eq!(body2, "");

    if method == Method::POST && pool == 42 {
        HttpResponse::Ok()
    } else {
        eprintln!("method: {method} | pool: {pool}");
        HttpResponse::NotImplemented()
    }
}

#[actix_web::test]
async fn tdd() {
    let srv = actix_test::start(|| {
        App::new()
            .app_data(43u64)
            .app_data(web::Data::new(42u32))
            .default_service(web::to(handler))
    });

    let res = srv.post("/").send_body("foo").await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[derive(Debug, FromRequest)]
struct GenericRequestParts<T: Clone, U, V>
where
    U: Default,
{
    pool: web::Data<T>,
    value: U,

    #[from_request(copy_from_app_data)]
    copied: V,
}

async fn generic_handler(parts: GenericRequestParts<u32, Method, u64>) -> impl Responder {
    assert_eq!(**parts.pool, 42);
    assert_eq!(parts.value, Method::POST);
    assert_eq!(parts.copied, 43);

    HttpResponse::Ok()
}

#[actix_web::test]
async fn generic_struct_extraction() {
    let srv = actix_test::start(|| {
        App::new()
            .app_data(43u64)
            .app_data(web::Data::new(42u32))
            .default_service(web::to(generic_handler))
    });

    let res = srv.post("/").send().await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}
