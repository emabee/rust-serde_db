use serde;
use std::fmt;
use std::vec;
use std::sync::Arc;

use super::super::{DbValue, DeserializableRow, DeserError};
use super::Metadata;

/// A generic implementation of a single line of a ResultSet.
#[derive(Clone)]
pub struct Row<MD: Metadata, TV: DbValue> {
    metadata: Arc<MD>,
    values: Vec<TV>,
}

impl<MD, TV> Row<MD, TV>
    where MD: Metadata,
          TV: DbValue
{
    /// Factory for row.
    pub fn new(metadata: Arc<MD>, values: Vec<TV>) -> Row<MD, TV> {
        Row {
            metadata: metadata,
            values: values,
        }
    }

    /// Returns a clone of the ith value.
    pub fn cloned_value(&self, i: usize) -> Result<TV, DeserError> {
        trace!("<Row as DeserializableRow>::get() called");
        self.values
            .get(i)
            .map(|tv| tv.clone())
            .ok_or(DeserError::Implementation("element with index {} does not exist".to_owned()))
    }

    /// Converts the field into a plain rust value.
    pub fn pop_into_typed<'de, T>(&mut self) -> Result<T, <Row<MD, TV> as DeserializableRow>::E>
        where T: serde::de::Deserialize<'de>,
              TV: DbValue
    {
        trace!("Row::pop_into_typed()");
        Ok(DbValue::into_typed(DeserializableRow::pop(self).unwrap())?)
    }

    /// Converts the field into a plain rust value.
    pub fn field_into_typed<'de, T>(&self,
                                    i: usize)
                                    -> Result<T, <Row<MD, TV> as DeserializableRow>::E>
        where T: serde::de::Deserialize<'de>
    {
        trace!("Row::field_into_typed()");
        Ok(DbValue::into_typed(self.cloned_value(i)?)?)
    }

    /// Converts the Row into a rust value.
    pub fn into_typed<'de, T>(self) -> Result<T, DeserError>
        where T: serde::de::Deserialize<'de>
    {
        trace!("Row::into_typed()");
        Ok(DeserializableRow::into_typed(self)?)
    }
}

impl<MD: Metadata, TV: DbValue> DeserializableRow for Row<MD, TV> {
    type V = TV;
    type E = DeserError;

    /// Returns the length of the row.
    fn len(&self) -> usize {
        trace!("<Row as DeserializableRow>::len() called");
        self.values.len()
    }

    /// Removes and returns the last value.
    fn pop(&mut self) -> Option<TV> {
        trace!("<Row as DeserializableRow>::pop() called");
        let result = self.values.pop();
        result
    }

    /// Returns a reference to the last value.
    fn last(&self) -> Option<&TV> {
        trace!("<Row as DeserializableRow>::last() called");
        self.values.last()
    }

    /// Returns the name of the column at the specified index
    fn get_fieldname(&self, field_idx: usize) -> Option<&String> {
        trace!("<Row as DeserializableRow>::get_fieldname() called");
        self.metadata.get_fieldname(field_idx)
    }

    /// Reverses the order of the values
    fn reverse_values(&mut self) {
        trace!("<Row as DeserializableRow>::reverse() called");
        self.values.reverse()
    }
}

impl<MD: Metadata, TV: DbValue> IntoIterator for Row<MD, TV> {
    type Item = TV;
    type IntoIter = vec::IntoIter<TV>;

    fn into_iter(self) -> Self::IntoIter {
        trace!("<Row as IntoIterator>::into_iter() called");
        self.values.into_iter()
    }
}

impl<MD: Metadata, TV: DbValue> fmt::Display for Row<MD, TV>
    where TV: fmt::Display
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for value in &self.values {
            fmt::Display::fmt(&value, fmt)?;
            write!(fmt, ", ")?;
        }
        Ok(())
    }
}
