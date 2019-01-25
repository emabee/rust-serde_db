use serde;
use std::convert::From;
use std::marker::Sized;

use crate::de::row_deserializer::RowDeserializer;
use crate::de::{DbValue, DeserializationError};

/// A minimal interface for the Row type to support the deserialization.
#[allow(clippy::len_without_is_empty)]
pub trait DeserializableRow: Sized {
    /// The error type used by the database driver.
    type E: From<DeserializationError> + Sized;
    /// The value type used by the database driver.
    type V: DbValue;

    /// Returns the current length of the row (which is decremented with each call to next()).
    fn len(&self) -> usize;

    /// Removes and returns the next value.
    fn next(&mut self) -> Option<Self::V>;

    /// Returns the number of fields in a complete row.
    fn number_of_fields(&self) -> usize;

    /// Returns the name of the column at the specified index.
    fn fieldname(&self, field_idx: usize) -> Option<&String>;

    /// Converts the row into a struct, a tuple, or (if applicable) into a plain rust value.
    fn into_typed<'de, T>(self) -> Result<T, Self::E>
    where
        T: serde::de::Deserialize<'de>,
    {
        Ok(serde::de::Deserialize::deserialize(
            &mut RowDeserializer::new(self),
        )?)
    }
}
