//! Support for deserializing database resultsets and rows into rust types.
//!
//! Support for serialization is planned.
//!
//! This crate uses serde, so it is fast and cheap.
//! The target types for deserialization need to implement <code>serde::de::Deserialize</code>,
//! what is automatically given for all elementary rust types and easily achieved for
//! custom structs (using <code>#[derive(Deserialize)]</code>).
//!
//! The deserialization of <code>serde_db</code> is designed such that
//! implementing database drivers can make use of it without impact on their customers.
//! They only need to enrich their implementations of resultset and row
//! with an additional method which delegates to the respective method provided
//! by this crate.

#![warn(missing_docs)]

#[macro_use]
extern crate log;
extern crate serde;

pub mod de;
