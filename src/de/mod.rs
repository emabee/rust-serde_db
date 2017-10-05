//! Support for deserializing database resultsets, or individual rows, or individual values,
//! into rust types.
//!
//! Implementing DB drivers just need to implement DeserializableResultset,
//! DeserializableRow, and -- more effort -- DbValue.
//!
//! We further recommend to implement the methods into_typed() directly on the
//! driver's classes for Resultset and Row with a plain delegation to the provided methods
//! DeserializableResultset::into_typed() and DeserializableRow::into_typed().
//!
//! This provides the functionality of this crate to the users of the DB driver without the need
//! of importing DeserializableResultset or DeserializableRow.
//!
//! # Examples
//!
//! Convert a n&#215;m resultset into a Vec of structs which implement serde::de::Deserialize:
//!
//! ```ignore
//! #[macro_use]
//! extern crate serde_derive;
//! ...
//! #[derive(Deserialize)]
//! struct MyStruct {...}
//! ...
//! let resultset = ...;
//! let data: Vec<MyStruct> = resultset.into_typed().unwrap();
//! ```
//!
//! Convert a n&#215;1 resultset into a Vec of fields:
//!
//! ```ignore
//! let vec_s: Vec<String> = resultset.into_typed().unwrap();
//! ```
//!
//! Convert a 1&#215;1 resultset into a single field:
//!
//! ```ignore
//! let s: String = resultset.into_typed().unwrap();
//! ```
//!
//! Loop over rows, convert each row individually into a struct
//! (for better streaming support with large result sets):
//!
//! ```ignore
//! for row in resultset {
//!     let data: MyStruct = row.into_typed().unwrap();
//! }
//! ```
//!
//! Or convert the rows into tuples (no need to derive serde::de::Deserialize):
//!
//! ```ignore
//! for row in resultset {
//!     let t: (String, NaiveDateTime, i32, Option<i32>) = row.into_typed().unwrap();
//! }
//! ```
//!
//! FIXME Add example for single field evaluation
//!

mod db_value;
mod conversion_error;
mod deserializable_resultset;
mod deserializable_row;
mod deserialization_error;
mod field_deserializer;
pub mod row;
mod row_deserializer;
mod rs_deserializer;

pub use de::conversion_error::ConversionError;
pub use self::deserialization_error::DeserError;

pub use self::db_value::{DbValue, DbValueInto};
pub use self::deserializable_resultset::DeserializableResultset;
pub use self::deserializable_row::DeserializableRow;
