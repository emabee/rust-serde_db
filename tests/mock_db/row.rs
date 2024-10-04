use serde;
use serde_db::de::{DbValue, DeserializableRow, DeserializationError};
use std::rc::Rc;

use crate::mock_db;

// A generic implementation of a single line of a `ResultSet`.
#[derive(Clone, Debug)]
pub struct Row {
    metadata: Rc<mock_db::FieldNames>,
    value_iter: <Vec<mock_db::MValue> as IntoIterator>::IntoIter,
}

impl Row {
    pub fn new(metadata: Rc<mock_db::FieldNames>, values: Vec<mock_db::MValue>) -> mock_db::Row {
        mock_db::Row {
            metadata: metadata,
            value_iter: values.into_iter(),
        }
    }

    // Returns a clone of the ith value.
    pub fn cloned_value(&self, i: usize) -> Result<mock_db::MValue, DeserializationError> {
        trace!("<mock_db::Row as DeserializableRow>::get()");
        Ok(self.value_iter.as_slice()[i].clone())
    }

    // Removes and converts the next field into a plain rust value.
    pub fn next_try_into<'de, T>(&mut self) -> Result<T, <mock_db::Row as DeserializableRow>::E>
    where
        T: serde::Deserialize<'de>,
    {
        trace!("Row::next_try_into()");
        Ok(DbValue::try_into(DeserializableRow::next(self).unwrap())?)
    }

    // Clones and converts the specified field into a plain rust value.
    pub fn field_into<'de, T>(&self, i: usize) -> mock_db::Result<T>
    where
        T: serde::Deserialize<'de>,
    {
        trace!("Row::field_into() for {:?}", self.value_iter.as_slice()[i]);
        Ok(DbValue::try_into(self.cloned_value(i)?)?)
    }

    pub fn next(&mut self) -> Option<mock_db::MValue> {
        trace!("mock_db::Row::next()");
        self.value_iter.next()
    }

    // Converts the complete Row into a rust value.
    pub fn try_into<'de, T>(self) -> mock_db::Result<T>
    where
        T: serde::Deserialize<'de>,
    {
        trace!("Row::try_into()");
        Ok(DeserializableRow::try_into(self)?)
    }
}

impl DeserializableRow for mock_db::Row {
    type V = mock_db::MValue;
    type E = mock_db::Error;

    fn len(&self) -> usize {
        trace!("<mock_db::Row as DeserializableRow>::len()");
        self.value_iter.as_slice().len()
    }

    fn next(&mut self) -> Option<mock_db::MValue> {
        trace!("<mock_db::Row as DeserializableRow>::next()");
        self.next()
    }

    fn number_of_fields(&self) -> usize {
        self.metadata.number_of_fields()
    }

    fn field_name(&self, field_idx: usize) -> Option<&str> {
        trace!("<mock_db::Row as DeserializableRow>::field_name()");
        self.metadata.field_name(field_idx)
    }
}

impl Iterator for mock_db::Row {
    type Item = mock_db::MValue;
    fn next(&mut self) -> Option<mock_db::MValue> {
        self.next()
    }
}
