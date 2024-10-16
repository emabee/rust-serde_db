//! Support for deserializing database result sets, and/or individual database rows,
//! and/or individual database values, into rust types.
//!
//! # Result sets
//!
//! Rather than iterating over rows and columns of a result set, applications
//! can convert result sets directly into their data structure of choice.
//!
//! This approach allows, in contrast to many ORM mapping variants, using the
//! full flexibility of SQL (projection lists, all kinds of joins, unions, etc, etc).
//! Whatever query an application needs, they just use it, and whatever result structure they need,
//! they just use a corresponding rust structure or tuple into which they deserialize the data
//! in a single method call.
//!
//! The target types for deserialization only need to implement
//! [`serde::Deserialize`](https://docs.serde.rs/serde/de/trait.Deserialize.html),
//! which is automatically given for all elementary rust types and tuples,
//! and easily achieved for custom structs using `#[derive(Deserialize)]`.
//!
//! Result sets can _always_ be deserialized into a `Vec<line_struct>`, where
//! `line_struct` matches the field list of the result set and
//! implements `serde::Deserialize`.
//!
//! Similarly, a `Vec<(...)>`, works as well, as long as the tuple
//! members match the field list of the result set.
//!
//! In addition, `serde_db` also supports structural simplification:
//! depending on the dimension of the result set, simplified target types can be
//! chosen for deserialization:
//!
//! * If the result set contains only a single line (e.g. because you specified
//!   TOP 1 in your select),
//!   then you can optionally choose to deserialize directly into a plain `line_struct`.
//!
//! * If the result set contains only a single column, then you can optionally choose to
//!   deserialize into a `Vec<plain_field>`.
//!
//! * If the result set contains only a single value (one row with one column),
//!   then you can optionally choose to deserialize into a plain `line_struct`,
//!   or a `Vec<plain_field>`, or a plain variable.
//!
//! The below examples assume the DB driver exposes on its
//! result set type a function
//! `fn try_into<'de, T: serde::Deserialize<'de>>(self) -> mock_db::Result<T>`,
//! which is implemented using `serde_db`.
//!
//! ## Convert a n×m result set into a Vec of structs:
//!
//! ```rust,ignore
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]  // This is all we need
//! struct MyStruct {
//!   // fields ... should match the columns of the result set
//! }
//!
//! // let result set = ...;
//! let data: Vec<MyStruct> = result set.try_into()?;
//! ```
//!
//!
//! ## Convert a n×1 result set into a Vec of fields:
//!
//! ```rust,ignore
//! // possible if the rows have a single field only
//! let vec_s: Vec<String> = result set.try_into()?;
//! ```
//!
//! ## Convert a 1×1 result set into a single field:
//!
//! ```rust,ignore
//! // possible if there is only one row with one field
//! let s: String = result set.try_into()?;
//! ```
//!
//! # Rows
//!
//! ## Convert rows into tuples or structs
//!
//! For better streaming of large result sets, you might want to iterate over the rows, like in
//!
//! ```rust,ignore
//! for row in result set {
//!     let t: (String, NaiveDateTime, i32, Option<i32>) = row.try_into()?;
//! }
//! ```
//!
//! or
//!
//! ```rust,ignore
//! for row in result set {
//!     let data: MyStruct = row.try_into()?;
//! }
//! ```
//!
//! # Individual values
//!
//! When necessary, you can also convert individual values directly into an adequate rust type:
//!
//! ```rust,ignore
//! for row in result set {
//!     let first_dbvalue = row.next().unwrap();
//!     let first: NaiveDateTime = first_dbvalue.try_into()?;
//!     // ...
//! }
//! ```
//!
//! # Note for implementors
//!
//! Implementing DB drivers need
//! to implement [`DeserializableResultSet`](trait.DeserializableResultSet.html) and
//! [`DeserializableRow`](trait.DeserializableRow.html), which are trivial,
//! and [`DbValue`](trait.DbValue.html), which is a bit more effort
//! (an example can be found in the tests of this crate), depending on the flexibility
//! you want to offer.
//!
//! We further recommend adding a method like `try_into()` directly on the
//! driver's class for result sets with a plain delegation to the _provided_ method
//! [`DeserializableResultSet::try_into()`](trait.DeserializableResultSet.html#method.try_into).
//! The same should be done for rows.
//! By this, the deserialization functionality of `serde_db` can be provided
//! to the users of the DB driver without forcing them to import `serde_db`.

mod conversion_error;
mod db_value;
mod db_value_into;
mod deserializable_result_set;
mod deserializable_row;
mod deserialization_error;
mod field_deserializer;
mod row_deserializer;
mod rs_deserializer;

pub use self::conversion_error::ConversionError;
pub use self::db_value::DbValue;
pub use self::db_value_into::DbValueInto;
pub use self::deserializable_result_set::DeserializableResultSet;
pub use self::deserializable_row::DeserializableRow;
pub use self::deserialization_error::{DeserializationError, DeserializationResult};
