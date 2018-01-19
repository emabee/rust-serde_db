use serde;
use serde_db::de::{DbValue, DeserializableRow, DeserializationError};
use std::vec;
use std::rc::Rc;

use mock_db;

/// A generic implementation of a single line of a `ResultSet`.
#[derive(Clone)]
pub struct Row {
    metadata: Rc<mock_db::Fieldnames>,
    values: Vec<mock_db::MValue>,
}

impl Row {
    pub fn new(metadata: Rc<mock_db::Fieldnames>, values: Vec<mock_db::MValue>) -> mock_db::Row {
        mock_db::Row {
            metadata: metadata,
            values: values,
        }
    }

    /// Returns a clone of the ith value.
    pub fn cloned_value(&self, i: usize) -> Result<mock_db::MValue, DeserializationError> {
        trace!("<mock_db::Row as DeserializableRow>::get()");
        self.values.get(i).cloned().ok_or_else(|| {
            DeserializationError::Implementation("element with index {} does not exist".to_owned())
        })
    }

    /// Converts the field into a plain rust value.
    pub fn pop_into_typed<'de, T>(&mut self) -> Result<T, <mock_db::Row as DeserializableRow>::E>
    where
        T: serde::de::Deserialize<'de>,
    {
        trace!("Row::pop_into_typed()");
        Ok(DbValue::into_typed(DeserializableRow::pop(self).unwrap())?)
    }

    /// Converts the field into a plain rust value.
<<<<<<< HEAD
    pub fn field_into<'de, T>(&self, i: usize) -> mock_db::Result<T>
    where
        T: serde::de::Deserialize<'de>,
    {
        trace!("Row::field_into() for {:?}", self.values[i]);
        Ok(DbValue::into_typed(self.cloned_value(i)?)?)
=======
    pub fn field_into_plain_type<'de, T>(&self, i: usize) -> mock_db::Result<T>
    where
        T: serde::de::Deserialize<'de>,
    {
        trace!("Row::field_into_plain_type() for {:?}", self.values[i]);
        if self.values[i].is_null() {
            Err(mock_db::Error::DESERIALIZATION(DeserializationError::SerdeError(
                "Row::field_into_plain_type() called on Null value".to_owned(),
            )))
        } else {
            Ok(DbValue::into_typed(self.cloned_value(i)?)?)
        }
    }

    /// Converts the field into a plain rust value.
    pub fn field_into_option<'de, T>(&self, i: usize) -> mock_db::Result<Option<T>>
    where
        T: serde::de::Deserialize<'de>,
    {
        trace!("Row::field_into_option() for {:?}", self.values[i]);
        if self.values[i].is_null() {
            Ok(None)
        } else {
            Ok(Some(DbValue::into_typed(self.cloned_value(i)?)?))
        }
>>>>>>> 145c8e08c0d38382d2e87b68dab203a778d6601d
    }

    /// Converts the mock_db::Row into a rust value.
    pub fn try_into<'de, T>(self) -> mock_db::Result<T>
    where
        T: serde::de::Deserialize<'de>,
    {
        trace!("Row::try_into()");
        Ok(DeserializableRow::into_typed(self)?)
    }
}

impl DeserializableRow for mock_db::Row {
    type V = mock_db::MValue;
    type E = mock_db::Error;

    /// Returns the length of the row.
    fn len(&self) -> usize {
        trace!("<mock_db::Row as DeserializableRow>::len()");
        self.values.len()
    }

    /// Removes and returns the last value.
    fn pop(&mut self) -> Option<mock_db::MValue> {
        trace!("<mock_db::Row as DeserializableRow>::pop()");
        self.values.pop()
    }

    /// Returns the name of the column at the specified index
    fn get_fieldname(&self, field_idx: usize) -> Option<&String> {
        trace!("<mock_db::Row as DeserializableRow>::get_fieldname()");
        self.metadata.get_fieldname(field_idx)
    }

    /// Reverses the order of the values
    fn reverse_values(&mut self) {
        trace!("<mock_db::Row as DeserializableRow>::reverse()");
        self.values.reverse()
    }
}

impl IntoIterator for mock_db::Row {
    type Item = mock_db::MValue;
    type IntoIter = vec::IntoIter<mock_db::MValue>;

    fn into_iter(self) -> Self::IntoIter {
        trace!("<mock_db::Row as IntoIterator>::into_iter()");
        self.values.into_iter()
    }
}
