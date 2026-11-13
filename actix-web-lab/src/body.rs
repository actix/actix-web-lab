//! Experimental body types.
//!
//! Analogous to the `body` module in Actix Web.

#[expect(deprecated)]
pub use crate::infallible_body_stream::{new_infallible_body_stream, new_infallible_sized_stream};
pub use crate::{
    body_async_write::{Writer, writer},
    body_channel::{Sender, channel},
};
