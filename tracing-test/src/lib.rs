//! Helper functions and macros that allow for easier testing of crates that use `tracing`.
//!
//! This crate should mainly be used through the [`#[traced_test]`](attr.traced_test.html) macro.
pub mod internal;
mod subscriber;

pub use tracing_test_macro::traced_test;
