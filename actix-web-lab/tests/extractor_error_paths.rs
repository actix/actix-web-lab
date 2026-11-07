//! Regression coverage for extractor diagnostics and input completion.

use actix_web::{FromRequest, ResponseError, http::StatusCode, test::TestRequest};
use actix_web_lab::extract::{Json, Query, UrlEncodedForm};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Input {
    n: u32,
    s: String,
    b: bool,
}

#[actix_web::test]
async fn errors_keep_paths_sources_and_status() {
    let err = Query::<Input>::from_query("n=123&s=hi&b=invalid").unwrap_err();
    assert_eq!(err.path().to_string(), "b");
    assert_eq!(err.to_string(), "Query deserialization failed at path: b");
    assert_eq!(err.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        err.source().to_string(),
        r#"invalid value: string "invalid", expected "true", "on" or "false""#
    );

    let (req, mut payload) = TestRequest::default()
        .insert_header(("content-type", "application/json"))
        .set_payload(r#"{"n":123,"s":"hi","b":42}"#)
        .to_http_parts();
    let err = Json::<Input>::from_request(&req, &mut payload)
        .await
        .unwrap_err();
    assert_eq!(err.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(err.to_string(), "Deserialization error");
    let actix_web_lab::extract::JsonPayloadError::Deserialize { source } = err else {
        panic!()
    };
    assert_eq!(source.path().to_string(), "b");
    assert_eq!(source.source().line(), 1);
    assert_eq!(source.source().column(), 24);
    assert_eq!(
        source.source().to_string(),
        "invalid type: integer `42`, expected a boolean at line 1 column 24"
    );

    let (req, mut payload) = TestRequest::default()
        .insert_header(("content-type", "application/x-www-form-urlencoded"))
        .set_payload("n=123&s=hi&b=invalid")
        .to_http_parts();
    let err = UrlEncodedForm::<Input>::from_request(&req, &mut payload)
        .await
        .unwrap_err();
    assert_eq!(
        err.as_response_error().status_code(),
        StatusCode::UNPROCESSABLE_ENTITY
    );
    assert_eq!(err.to_string(), "Deserialization error");
}

#[actix_web::test]
async fn json_keeps_trailing_input_behavior() {
    let (req, mut payload) = TestRequest::default()
        .insert_header(("content-type", "application/json"))
        .set_payload(r#"{"n":123,"s":"hi","b":false} trailing"#)
        .to_http_parts();
    let value = Json::<Input>::from_request(&req, &mut payload)
        .await
        .unwrap();
    assert_eq!(value.n, 123);
    assert_eq!(value.s, "hi");
    assert!(!value.b);
}

#[actix_web::test]
async fn retry_can_succeed_for_a_stateful_deserializer() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static CALLS: AtomicUsize = AtomicUsize::new(0);

    #[derive(Debug)]
    struct Retry;

    impl<'de> Deserialize<'de> for Retry {
        fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
            let _ = serde::de::IgnoredAny::deserialize(de)?;
            if CALLS.fetch_add(1, Ordering::SeqCst).is_multiple_of(2) {
                Err(serde::de::Error::custom("first attempt fails"))
            } else {
                Ok(Self)
            }
        }
    }

    Query::<Retry>::from_query("n=123").unwrap();

    let (req, mut payload) = TestRequest::default()
        .insert_header(("content-type", "application/json"))
        .set_payload("{}")
        .to_http_parts();
    Json::<Retry>::from_request(&req, &mut payload)
        .await
        .unwrap();

    let (req, mut payload) = TestRequest::default()
        .insert_header(("content-type", "application/x-www-form-urlencoded"))
        .set_payload("n=123")
        .to_http_parts();
    UrlEncodedForm::<Retry>::from_request(&req, &mut payload)
        .await
        .unwrap();
    assert_eq!(CALLS.load(Ordering::SeqCst), 6);
}

#[actix_web::test]
async fn json_syntax_and_eof_errors_remain_bad_requests() {
    for body in [r#"{"n":!}"#, r#"{"n":123,"s":"hi","b":"#] {
        let (req, mut payload) = TestRequest::default()
            .insert_header(("content-type", "application/json"))
            .set_payload(body)
            .to_http_parts();
        let err = Json::<Input>::from_request(&req, &mut payload)
            .await
            .unwrap_err();
        assert_eq!(err.status_code(), StatusCode::BAD_REQUEST);
    }
}
