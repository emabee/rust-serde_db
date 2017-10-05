use serde;
use std::fmt;
use std::marker::Sized;

use super::rs_deserializer::RsDeserializer;
use super::deserializable_row::DeserializableRow;
use super::deserialization_error::DeserError;

/// Interface for a database resultset to support deserialization.
pub trait DeserializableResultset: fmt::Debug + Sized {
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
    /// (which implements serde::de::Deserialize).
    /// FIXME check this doc part
    /// A resultset is essentially a two-dimensional structure, given as a list of rows
    /// (a <code>Vec&lt;Row&gt;</code>),
    /// where each row is a list of fields (a <code>Vec&lt;TypedValue&gt;</code>);
    /// the name of each field is given in the metadata of the resultset.
    ///
    /// The method supports a variety of target data structures, with the only strong limitation
    /// that no data loss is supported.
    ///
    /// * It depends on the dimension of the resultset what target data structure
    ///   you can choose for deserialization:
    ///
    ///     * You can always use a <code>Vec&lt;line_struct&gt;</code>, where
    ///       <code>line_struct</code> matches the field list of the resultset.
    ///
    ///     * If the resultset contains only a single line (e.g. because you specified
    ///       TOP 1 in your select),
    ///       then you can optionally choose to deserialize into a plain <code>line_struct</code>.
    ///
    ///     * If the resultset contains only a single column, then you can optionally choose to
    ///       deserialize into a <code>Vec&lt;plain_field&gt;</code>.
    ///
    ///     * If the resultset contains only a single value (one row with one column),
    ///       then you can optionally choose to deserialize into a plain <code>line_struct</code>,
    ///       or a <code>Vec&lt;plain_field&gt;</code>, or a plain variable.
    ///
    /// * Also the translation of the individual field values provides a lot of flexibility.
    ///   You can e.g. convert values from a nullable column into a plain field,
    ///   provided that no NULL values are given in the resultset.
    ///
    ///   Vice versa, you always can use an Option<code>&lt;plain_field&gt;</code>,
    ///   even if the column is marked as NOT NULL.
    ///
    /// * Similarly, integer types can differ, as long as the concrete values can
    ///   be assigned without loss.
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
