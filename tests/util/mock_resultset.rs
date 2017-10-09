use serde;
use serde_db::de::{DeserializationError, DeserializableResultset};
use std::rc::Rc;

use super::*;

pub struct MockResultset {
    rows: Vec<MockRow>,
    md: Rc<MockMetadata>,
}
impl MockResultset {
    pub fn new(fields: Vec<&'static str>) -> MockResultset {
        MockResultset {
            rows: Vec::<MockRow>::new(),
            md: Rc::new(MockMetadata::new(fields)),
        }
    }
    pub fn push(&mut self, values: Vec<MockValue>) {
        assert_eq!(self.md.number_of_fields(), values.len());
        self.rows.push(MockRow::new(self.md.clone(), values))
    }

    // Expose the capability from serde_db
    pub fn into_typed<'de, T>(self) -> Result<T, MockError>
        where T: serde::de::Deserialize<'de>
    {
        trace!("MockResultset::into_typed()");
        Ok(DeserializableResultset::into_typed(self)?)
    }
}

impl DeserializableResultset for MockResultset {
    type E = MockError;
    type ROW = MockRow;
    fn has_multiple_rows(&mut self) -> Result<bool, DeserializationError> {
        Ok(self.rows.len() > 1_usize)
    }

    fn reverse_rows(&mut self) {
        self.rows.reverse()
    }

    fn pop_row(&mut self) -> Result<Option<<Self as DeserializableResultset>::ROW>, DeserializationError> {
        Ok(self.rows.pop())
    }

    fn number_of_fields(&self) -> usize {
        self.md.number_of_fields()
    }

    fn get_fieldname(&self, i: usize) -> Option<&String> {
        self.md.get_fieldname(i)
    }

    fn fetch_all(&mut self) -> Result<(), <Self as DeserializableResultset>::E> {
        Ok(())
    }
}

impl IntoIterator for MockResultset {
    type Item = MockRow;
    type IntoIter = <Vec<MockRow> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.rows.into_iter()
    }
}
