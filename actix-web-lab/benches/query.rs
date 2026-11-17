#![allow(missing_docs)]

use actix_web_lab::extract::Query;
use divan::{AllocProfiler, Bencher, black_box};
use serde::Deserialize;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

#[derive(Deserialize)]
struct Params {
    page: u32,
    tag: Vec<String>,
}

#[divan::bench]
fn parse_query(b: Bencher<'_, '_>) {
    let query = "page=2&tag=rust&tag=web%20development";
    let params = Query::<Params>::from_query(query).unwrap();

    assert_eq!(params.page, 2);
    assert_eq!(params.tag, ["rust", "web development"]);

    b.bench(|| Query::<Params>::from_query(black_box(query)).unwrap());
}

fn main() {
    divan::main();
}
