//! Support for deserializing database resultsets, or individual database rows,
//! or individual database values, into rust types.
//!
//! Concretely, we propose and enable using serde to simplify the data exchange
//! between applications and the db driver, for the
//! results that are returned from the database: there is no need to iterate over
//! a complex resultset by rows and columns!
//!
//! This approach allows, in contrast to many ORM mapping variants, using the
//! full flexibility of SQL (projection lists, all kinds of joins, unions, etc, etc).
//! Whatever query an application needs, they just use it, and whatever result structure they need,
//! they just use a corresponding rust structure or tuple into which they deserialize the data
//! in a single method call.
//!
//! The target types for deserialization need to implement [<code>serde::de::Deserialize</code>]
//! (https://docs.serde.rs/serde/de/trait.Deserialize.html),
//! what is automatically given for all elementary rust types and easily achieved for
//! custom structs (using <code>#[derive(Deserialize)]</code>).
//!
//! Depending on the dimension of the resultset, different target types can be
//! chosen for deserialization:
//!
//! * Resultsets can _always_ be deserialized into a <code>Vec&lt;line_struct&gt;</code>, where
//!   <code>line_struct</code> matches the field list of the resultset and
//!   implements <code>serde::de::Deserialize</code>.
//!
//! * Similarly, a <code>Vec&lt;(...)&gt;</code>, works as well, as long as the tuple
//!   members match the field list of the resultset.
//!
//! * If the resultset contains only a single line (e.g. because you specified
//!   TOP 1 in your select),
//!   then you can optionally choose to deserialize directly into a plain <code>line_struct</code>.
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
//! let data: Vec<MyStruct> = resultset.into_typed()?;
//! ```
//!
//! Or convert the rows into tuples (no need to implement serde::de::Deserialize):
//!
//! ```ignore
//! for row in resultset {
//!     let t: (String, NaiveDateTime, i32, Option<i32>) = row.into_typed()?;
//! }
//! ```
//!
//! Convert a n&#215;1 resultset into a Vec of fields:
//!
//! ```ignore
//! let vec_s: Vec<String> = resultset.into_typed()?;
//! ```
//!
//! Convert a 1&#215;1 resultset into a single field:
//!
//! ```ignore
//! let s: String = resultset.into_typed()?;
//! ```
//!
//! Loop over rows, convert each row individually into a struct
//! (for better streaming support with large result sets):
//!
//! ```ignore
//! for row in resultset {
//!     let data: MyStruct = row.into_typed()?;
//! }
//! ```
//!
//! # Note for implementors
//!
//! Implementing DB drivers need
//! to implement [<code>DeserializableResultset</code>](trait.DeserializableResultset.html),
//! [<code>DeserializableRow</code>](trait.DeserializableRow.html),
//! and -- a bit more effort --
//! [<code>DbValue</code>](trait.DbValue.html).
//!
//! We further recommend adding a method like <code>into_typed()</code> directly on the
//! driver's class for resultsets with a plain delegation to the _provided_ method
//! [<code>DeserializableResultset::into_typed()</code>](trait.DeserializableResultset.html#method.into_typed).
//! The same should be done for rows.
//!
//! By this, the deserialization functionality of <code>serde_db</code> can be provided
//! to the users of the DB driver without the necessity to import additional traits.
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
