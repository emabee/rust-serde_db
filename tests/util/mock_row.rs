use serde;
use serde_db::de::{DbValue, DeserializableRow, DeserializationError};
use std::vec;
use std::rc::Rc;

use super::{MockError, MockResult, MockValue};

/// A generic implementation of a single line of a ResultSet.
#[derive(Clone)]
pub struct MockRow {
    metadata: Rc<MockMetadata>,
    values: Vec<MockValue>,
}

impl MockRow {
    /// Factory for row.
    pub fn new(metadata: Rc<MockMetadata>, values: Vec<MockValue>) -> MockRow {
        MockRow {
            metadata: metadata,
            values: values,
        }
    }

    /// Returns a clone of the ith value.
    pub fn cloned_value(&self, i: usize) -> Result<MockValue, DeserializationError> {
        trace!("<MockRow as DeserializableRow>::get()");
        self.values
            .get(i)
            .map(|tv| tv.clone())
            .ok_or(DeserializationError::Implementation("element with index {} does not exist".to_owned()))
    }

    /// Converts the field into a plain rust value.
    pub fn pop_into_typed<'de, T>(&mut self) -> Result<T, <MockRow as DeserializableRow>::E>
        where T: serde::de::Deserialize<'de>
    {
        trace!("Row::pop_into_typed()");
        Ok(DbValue::into_typed(DeserializableRow::pop(self).unwrap())?)
    }

    /// Converts the field into a plain rust value.
    pub fn field_into_typed<'de, T>(&self, i: usize) -> MockResult<T>
        where T: serde::de::Deserialize<'de>
    {
        trace!("Row::field_into_typed()");
        Ok(DbValue::into_typed(self.cloned_value(i)?)?)
    }

    /// Converts the MockRow into a rust value.
    pub fn into_typed<'de, T>(self) -> MockResult<T>
        where T: serde::de::Deserialize<'de>
    {
        trace!("Row::into_typed()");
        Ok(DeserializableRow::into_typed(self)?)
    }
}

impl DeserializableRow for MockRow {
    type V = MockValue;
    type E = MockError;

    /// Returns the length of the row.
    fn len(&self) -> usize {
        trace!("<MockRow as DeserializableRow>::len()");
        self.values.len()
    }

    /// Removes and returns the last value.
    fn pop(&mut self) -> Option<MockValue> {
        trace!("<MockRow as DeserializableRow>::pop()");
        let result = self.values.pop();
        result
    }

    /// Returns the name of the column at the specified index
    fn get_fieldname(&self, field_idx: usize) -> Option<&String> {
        trace!("<MockRow as DeserializableRow>::get_fieldname()");
        self.metadata.get_fieldname(field_idx)
    }

    /// Reverses the order of the values
    fn reverse_values(&mut self) {
        trace!("<MockRow as DeserializableRow>::reverse()");
        self.values.reverse()
    }
}

impl IntoIterator for MockRow {
    type Item = MockValue;
    type IntoIter = vec::IntoIter<MockValue>;

    fn into_iter(self) -> Self::IntoIter {
        trace!("<MockRow as IntoIterator>::into_iter()");
        self.values.into_iter()
    }
}

pub struct MockMetadata {
    fields: Vec<String>,
}
impl MockMetadata {
    pub fn new(fields: Vec<&'static str>) -> MockMetadata {
        MockMetadata { fields: fields.iter().map(|s| String::from(*s)).collect() }
    }

    pub fn number_of_fields(&self) -> usize {
        self.fields.len()
    }

    pub fn get_fieldname(&self, i: usize) -> Option<&String> {
        self.fields.get(i)
    }
}
