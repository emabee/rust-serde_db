use serde;
use std::marker::Sized;

use de::rs_deserializer::RsDeserializer;
use de::{DeserializationError, DeserializationResult, DeserializableRow};

/// Interface for a database resultset to support deserialization.
pub trait DeserializableResultset: Sized {
    /// Error type of the database driver.
    type E: From<DeserializationError> + Sized;
    /// Concrete type for the DB row, which must implement DeserializabeRow.
    type ROW: DeserializableRow;

    /// Returns true if more than one row is contained (implementors should consider
    /// eventually not yet fetched rows)
    fn has_multiple_rows(&mut self) -> DeserializationResult<bool>;

    /// Reverses the order of the rows
    fn reverse_rows(&mut self);

    /// Removes the last row and returns it, or None if it is empty, or an error.
    fn pop_row(&mut self) -> DeserializationResult<Option<Self::ROW>>;

    /// Returns the number of fields in each row
    fn number_of_fields(&self) -> usize;

    /// Returns the name of the column at the specified index
    fn get_fieldname(&self, field_idx: usize) -> Option<&String>;

    /// Fetches all not yet transported result rows from the server.
    ///
    /// Bigger resultsets may be transported in multiple DB roundtrips;
    /// not yet fetched rows are typically fetched on demand; after this method,
    /// the resultset is expected to be loaded completely.
    fn fetch_all(&mut self) -> Result<(), Self::E>;

    /// A _provided method_ that translates a resultset into a given rust type
    /// that implements serde::de::Deserialize.
    ///
    /// The type of the target variable needs to be specified explicitly, so that
    /// <code>into_typed()</code> can derive the type it needs to serialize into:
    ///
    /// ```ignore
    /// #[derive(Deserialize)]
    /// struct MyStruct {
    ///     ...
    /// }
    /// let typed_result: Vec<MyStruct> = resultset.into_typed()?;
    /// ```
    fn into_typed<'de, T>(mut self) -> Result<T, Self::E>
        where T: serde::de::Deserialize<'de>,
              Self: Sized
    {
        self.fetch_all()?;
        Ok(serde::de::Deserialize::deserialize(&mut RsDeserializer::new(self)?)?)
    }
}
