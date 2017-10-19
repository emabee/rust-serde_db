//! Support for deserializing database resultsets and rows into rust types,
//! and Support for serializing rust types into database parameter values.
//!
//! This crate uses serde, so it is fast and cheap and easy to use.

#![warn(missing_docs)]

#[macro_use]
extern crate log;
extern crate serde;

pub mod de;
pub mod ser;

pub use ser::to_params;
