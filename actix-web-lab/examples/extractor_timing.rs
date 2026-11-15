//! Local extractor timing harness. No sockets or benchmark service are used.
use std::{
    hint::black_box,
    time::{Duration, Instant},
};

use actix_web::{FromRequest, test::TestRequest};
use actix_web_lab::extract::{Json, Query, UrlEncodedForm};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Input {
    n: u32,
    s: String,
    b: bool,
}

#[actix_web::main]
async fn main() {
    let case = std::env::args()
        .nth(1)
        .expect("case: json/query/form-success/early/late");
    let (kind, mode) = case.split_once('-').unwrap();
    let json = match mode {
        "success" => r#"{"n":123,"s":"hi there","b":false}"#,
        "early" => r#"{"n":"bad","s":"hi there","b":false}"#,
        "late" => r#"{"n":123,"s":"hi there","b":"bad"}"#,
        _ => panic!("unknown mode"),
    };
    let form = match mode {
        "success" => "n=123&s=hi+there&b=false",
        "early" => "n=bad&s=hi+there&b=false",
        "late" => "n=123&s=hi+there&b=bad",
        _ => unreachable!(),
    };
    for measured in [false, true] {
        let start = Instant::now();
        let mut count = 0_u64;
        while start.elapsed() < Duration::from_secs(1) {
            for _ in 0..100 {
                let value = match kind {
                    "query" => Query::<Input>::from_query(black_box(form))
                        .map(|v| v.into_inner())
                        .map_err(|err| black_box(err.to_string())),
                    "json" => {
                        let (req, mut pl) = TestRequest::default()
                            .insert_header(("content-type", "application/json"))
                            .set_payload(black_box(json))
                            .to_http_parts();
                        Json::<Input>::from_request(&req, &mut pl)
                            .await
                            .map(|v| v.into_inner())
                            .map_err(|err| black_box(err.to_string()))
                    }
                    "form" => {
                        let (req, mut pl) = TestRequest::default()
                            .insert_header(("content-type", "application/x-www-form-urlencoded"))
                            .set_payload(black_box(form))
                            .to_http_parts();
                        UrlEncodedForm::<Input>::from_request(&req, &mut pl)
                            .await
                            .map(|v| v.into_inner())
                            .map_err(|err| black_box(err.to_string()))
                    }
                    _ => panic!("unknown extractor"),
                };
                assert_eq!(value.is_ok(), mode == "success");
                if let Ok(value) = value {
                    assert_eq!(value.n, 123);
                    assert_eq!(value.s, "hi there");
                    assert!(!value.b);
                    black_box(value);
                }
                count += 1;
            }
        }
        if measured {
            println!("{case},{count},{}", start.elapsed().as_nanos());
        }
    }
}
