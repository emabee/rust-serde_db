use crate::de::rs_deserializer::RsDeserializer;
use crate::de::{DeserializableRow, DeserializationError, DeserializationResult};
use std::marker::Sized;

/// Interface for a database result set to support deserialization.
pub trait DeserializableResultSet: Sized {
    /// Error type of the database driver.
    type Error: From<DeserializationError> + Sized;
    /// Concrete type for the DB row, which must implement `DeserializabeRow`.
    type Row: DeserializableRow;

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
    fn next(&mut self) -> DeserializationResult<Option<Self::Row>>;

    /// Returns the number of fields in each (complete) row.
    fn number_of_fields(&self) -> usize;

    /// Returns the name of the column at the specified index.
    fn field_name(&self, field_idx: usize) -> Option<&str>;

    /// A _provided method_ that translates a result set into a given rust type
    /// that implements `serde::Deserialize`.
    ///
    /// The type of the target variable needs to be specified explicitly, so that
    /// `try_into()` can derive the type it needs to serialize into:
    ///
    /// ```ignore
    /// #[derive(Deserialize)]
    /// struct MyStruct {
    ///     ...
    /// }
    /// let typed_result: Vec<MyStruct> = result_set.try_into()?;
    /// ```
    ///
    /// # Errors
    ///
    /// An error is produced if deserialization into the target type is not possible,
    /// or if fetching fails.
    fn try_into<'de, T>(self) -> Result<T, Self::Error>
    where
        T: serde::Deserialize<'de>,
    {
        #[cfg(feature = "trace")]
        log::trace!("DeserializableResultSet::try_into()");
        Ok(serde::Deserialize::deserialize(
            &mut RsDeserializer::try_new(self)?,
        )?)
    }
}
