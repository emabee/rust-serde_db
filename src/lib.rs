//! Support for deserializing database resultsets and rows into rust types.
//!
//! (Support for serialization is planned.)
//!
//! This crate uses serde, so it is fast and cheap and easy to use.

#![warn(missing_docs)]

#[macro_use]
extern crate log;
extern crate serde;

pub mod de;
