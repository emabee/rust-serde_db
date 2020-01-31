use serde;
use std::marker::Sized;

use crate::de::rs_deserializer::RsDeserializer;
use crate::de::{DeserializableRow, DeserializationError, DeserializationResult};

/// Interface for a database resultset to support deserialization.
pub trait DeserializableResultset: Sized {
    /// Error type of the database driver.
    type E: From<DeserializationError> + Sized;
    /// Concrete type for the DB row, which must implement `DeserializabeRow`.
    type ROW: DeserializableRow;

    /// Returns true if more than one row is contained, including eventually not yet fetched rows.
    ///
    /// # Errors
    ///
    /// E.g. fetching can fail.
    fn has_multiple_rows(&mut self) -> DeserializationResult<bool>;

    /// Removes the next row and returns it, or None if the result set is empty, or an error.
    ///
    /// # Errors
    ///
    /// E.g. fetching can fail.
    fn next(&mut self) -> DeserializationResult<Option<Self::ROW>>;

    /// Returns the number of fields in each (complete) row.
    fn number_of_fields(&self) -> usize;

    /// Returns the name of the column at the specified index.
    fn fieldname(&self, field_idx: usize) -> Option<&str>;

    /// A _provided method_ that translates a resultset into a given rust type
    /// that implements `serde::de::Deserialize`.
    ///
    /// The type of the target variable needs to be specified explicitly, so that
    /// `into_typed()` can derive the type it needs to serialize into:
    ///
    /// ```ignore
    /// #[derive(Deserialize)]
    /// struct MyStruct {
    ///     ...
    /// }
    /// let typed_result: Vec<MyStruct> = resultset.into_typed()?;
    /// ```
    ///
    /// # Errors
    ///
    /// An error is produced if deserialization into the target type is not possible,
    /// or if fetching fails.
    fn into_typed<'de, T>(self) -> Result<T, Self::E>
    where
        T: serde::de::Deserialize<'de>,
        Self: Sized,
    {
        Ok(serde::de::Deserialize::deserialize(
            &mut RsDeserializer::try_new(self)?,
        )?)
    }
}
