//! Support for serializing rust types into parameters for database commands.
//!
//! Implementing DB drivers can make use of this functionality when they
//! need to translate rust values into their DB types.
//!
//! Prepared statements for example might have a function
//!
//! ```rust,ignore
//! fn add_batch<T>(&mut self, input: &T) -> HdbResult<()>
//! where T: serde::ser::Serialize
//! ```
//!
//! The consumers of the `add_batch()` function can hand over
//! a _tuple_ of rust values that correspond to the parameters the prepared statement
//! needs. Or they can hand over an appropriate _struct_ that implements `serde::ser::Serialize`.
//!
//! In both cases they do not need to differentiate between nullable and non-nullable
//! database values (except that they cannot convert an `Option::None` into
//! a non-nullable database value).
//!
//! In its implementation of [`DbvFactory`](trait.DbvFactory.html),
//! the DB driver can decide to make their life even easier
//! by converting flexibly between
//! different number types (an example can be found in the tests of this crate)
//!
//! The implementation of `add_batch()` converts `input` into
//! a Vec of the driver's database values that can subsequently be sent to the DB server:
//!
//! ```rust,ignore
//!     let db_values: Vec<DBValue> = serde_db::ser::to_params(&input, input_metadata)?;
//! ```
//!
//! It is assumed that the prepared statement has metadata about the required input parameters,
//! which implement [`DbvFactory`](trait.DbvFactory.html).

mod dbv_factory;
mod serialization_error;
mod serializer;

pub use self::dbv_factory::DbvFactory;
pub use self::serialization_error::SerializationError;


use serde;
use self::serializer::Serializer;

/// Provided method that translates the input into a Vec of database values.
///
/// Database drivers use this method in their implementation (e.g. behind a
/// `PreparedStatement::add_batch()`).
pub fn to_params<T: ?Sized, DF: DbvFactory>(value: &T, metadata: &[DF])
                                            -> Result<Vec<DF::DBV>, SerializationError>
where
    T: serde::ser::Serialize,
{
    trace!("serde_db::to_params()");
    let mut serializer = Serializer::new(metadata);
    value.serialize(&mut serializer)?;
    Ok(serializer.into_inner())
}
