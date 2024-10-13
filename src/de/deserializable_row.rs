use crate::de::row_deserializer::RowDeserializer;
use crate::de::{DbValue, DeserializationError};
use std::convert::From;
use std::marker::Sized;

/// A minimal interface for the Row type to support the deserialization.
#[allow(clippy::len_without_is_empty)]
pub trait DeserializableRow: Sized {
    /// The error type used by the database driver.
    type Error: From<DeserializationError> + Sized;
    /// The value type used by the database driver.
    type Value: DbValue;

    /// Returns the current length of the row (which is decremented with each call to next()).
    fn len(&self) -> usize;

    /// Removes and returns the next value.
    fn next(&mut self) -> Option<Self::Value>;

    /// Returns the number of fields in a complete row.
    fn number_of_fields(&self) -> usize;

    /// Returns the name of the column at the specified index.
    fn field_name(&self, field_idx: usize) -> Option<&str>;

    /// Converts the row into a struct, a tuple, or (if applicable) into a plain rust value.
    ///
    /// # Errors
    ///
    /// An error is produced if deserialization into the target type is not possible.
    fn try_into<'de, T>(self) -> Result<T, Self::Error>
    where
        T: serde::Deserialize<'de>,
    {
        Ok(serde::Deserialize::deserialize(&mut RowDeserializer::new(
            self,
        ))?)
    }
}
