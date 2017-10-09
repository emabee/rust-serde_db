//! Support for deserializing database resultsets, or individual database rows,
//! or individual database values, into rust types.
//!
//! The target types for deserialization need to implement <code>serde::de::Deserialize</code>,
//! what is automatically given for all elementary rust types and easily achieved for
//! custom structs (using <code>#[derive(Deserialize)]</code>).
//!
//! It depends on the dimension of the resultset what target data structure you can
//! choose for deserialization:
//!
//! * Resultsets can always be deserialized into a <code>Vec&lt;line_struct&gt;</code>, where
//!   <code>line_struct</code> matches the field list of the resultset.
//!
//! * Similarly, a <code>Vec&lt;(...)&gt;</code>, works as well, as long as the tuple
//!   members match the field list of the resultset.
//!
//! * If the resultset contains only a single line (e.g. because you specified
//!   TOP 1 in your select),
//!   then you can optionally choose to deserialize into a plain <code>line_struct</code>.
//!
//! * If the resultset contains only a single column, then you can optionally choose to
//!   deserialize into a <code>Vec&lt;plain_field&gt;</code>.
//!
//! * If the resultset contains only a single value (one row with one column),
//!   then you can optionally choose to deserialize into a plain <code>line_struct</code>,
//!   or a <code>Vec&lt;plain_field&gt;</code>, or a plain variable.
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
//! # Note for implementors
//!
//! Implementing DB drivers just need
//! to implement [<code>DeserializableResultset</code>](trait.DeserializableResultset.html),
//! [<code>DeserializableRow</code>](trait.DeserializableRow.html),
//! and -- a bit more effort --
//! [<code>DbValue</code>](trait.DbValue.html).
//!
//! We further recommend implementing a method like <code>into_typed()</code> directly on the
//! driver's class for resultsets with a plain delegation to the provided method
//! [<code>DeserializableResultset::into_typed()</code>](trait.DeserializableResultset.html#method.into_typed).
//! The same should be done for rows.
//!
//! By this extension of the driver's API, the deserialization functionality of
//! <code>serde_db</code> can be provided to the users of the DB driver without the need of
//! importing additional traits.
//!

mod db_value;
mod conversion_error;
mod deserializable_resultset;
mod deserializable_row;
mod deserialization_error;
mod field_deserializer;
mod row_deserializer;
mod rs_deserializer;

pub use self::conversion_error::ConversionError;
pub use self::db_value::{DbValue, DbValueInto};
pub use self::deserialization_error::{DeserializationError, DeserializationResult};
pub use self::deserializable_resultset::DeserializableResultset;
pub use self::deserializable_row::DeserializableRow;
