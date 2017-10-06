use serde;
use std::marker::Sized;

use super::rs_deserializer::RsDeserializer;
use super::deserializable_row::DeserializableRow;
use super::deserialization_error::DeserError;

/// Interface for a database resultset to support deserialization.
pub trait DeserializableResultset: Sized {
    /// Error type of the database driver.
    type E: From<DeserError> + Sized;
    /// Concrete type for the DB row, which must implement DeserializabeRow.
    type ROW: DeserializableRow;

    /// Returns true if more than one row is contained (implementors should consider
    /// eventually not yet fetched rows)
    fn has_multiple_rows(&mut self) -> Result<bool, DeserError>;

    /// Reverses the order of the rows
    fn reverse_rows(&mut self);

    /// Removes the last row and returns it, or None if it is empty, or an error.
    fn pop_row(&mut self) -> Result<Option<Self::ROW>, DeserError>;

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

    /// a _provided method_ that translates a generic resultset into a given rust type
    /// that implements serde::de::Deserialize.
    ///
    /// Note that you need to specify the type of your target variable explicitly, so that
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
        trace!("DeserializableResultSet::into_typed()");
        self.fetch_all()?;
        Ok(serde::de::Deserialize::deserialize(&mut RsDeserializer::new(self)?)?)
    }
}
