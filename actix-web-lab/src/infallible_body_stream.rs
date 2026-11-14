use actix_http::body::{BodyStream, SizedStream};
use bytes::Bytes;
use futures_util::Stream;

use crate::util::InfallibleStream;

/// Constructs a new [`BodyStream`] from an infallible byte chunk stream.
///
/// Use the upstream `actix_web::body::BodyStream::from_infallible()` constructor instead.
#[deprecated(
    since = "0.29.6",
    note = "Use the upstream `actix_web::body::BodyStream::from_infallible()` constructor instead."
)]
pub fn new_infallible_body_stream<S: Stream<Item = Bytes>>(
    stream: S,
) -> BodyStream<InfallibleStream<S>> {
    BodyStream::new(InfallibleStream::new(stream))
}

/// Constructs a new [`SizedStream`] from an infallible byte chunk stream.
///
/// Use the upstream `actix_web::body::SizedStream::from_infallible()` constructor instead.
#[deprecated(
    since = "0.29.6",
    note = "Use the upstream `actix_web::body::SizedStream::from_infallible()` constructor instead."
)]
pub fn new_infallible_sized_stream<S: Stream<Item = Bytes>>(
    size: u64,
    stream: S,
) -> SizedStream<InfallibleStream<S>> {
    SizedStream::new(size, InfallibleStream::new(stream))
}
