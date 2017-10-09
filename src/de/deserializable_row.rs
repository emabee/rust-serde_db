use serde;
use std::convert::From;
use std::marker::Sized;

use de::{DeserializationError, DbValue};
use de::row_deserializer::RowDeserializer;

/// A minimal interface for the Row type to support the deserialization.
pub trait DeserializableRow: Sized {
    /// The error type used by the database driver.
    type E: From<DeserializationError> + Sized;
    /// The value type used by the database driver.
    type V: DbValue;

    /// Returns the length of the row.
    fn len(&self) -> usize;

    /// Removes and returns the last value.
    fn pop(&mut self) -> Option<Self::V>;

    /// Returns the name of the column at the specified index
    fn get_fieldname(&self, field_idx: usize) -> Option<&String>;

    /// Reverses the order of the values. This method
    /// will be before deserialization of the row into a tuple starts,
    /// which uses pop() to access individual values.
    fn reverse_values(&mut self);

    /// Converts the row into a struct, a tuple, or (if applicable) into a plain rust value.
    fn into_typed<'de, T>(self) -> Result<T, Self::E>
        where T: serde::de::Deserialize<'de>
    {
        Ok(serde::de::Deserialize::deserialize(&mut RowDeserializer::new(self))?)
    }
}
