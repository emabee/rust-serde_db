use serde;

use serde_db::de::{DeserError, DeserializableResultset};
use serde_db::de::row::{Row, Metadata};

use std::fmt;
use std::sync::Arc;
use super::*;

pub struct MockResultset {
    rows: Vec<Row<MockMetadata, MockValue>>,
    md: Arc<MockMetadata>,
}
impl MockResultset {
    pub fn new(fields: Vec<&'static str>) -> MockResultset {
        MockResultset {
            rows: Vec::<Row<MockMetadata, MockValue>>::new(),
            md: Arc::new(MockMetadata::new(fields)),
        }
    }
    pub fn push(&mut self, values: Vec<MockValue>) {
        assert_eq!(self.md.number_of_fields(), values.len());
        self.rows.push(Row::new(self.md.clone(), values))
    }

    /// Converts the ResultSet into a rust value.
    pub fn into_typed<'de, T>(self) -> Result<T, MockError>
        where T: serde::de::Deserialize<'de>
    {
        trace!("MockResultset::into_typed()");
        Ok(DeserializableResultset::into_typed(self)?)
    }
}

impl DeserializableResultset for MockResultset {
    type E = MockError;
    type ROW = Row<MockMetadata, MockValue>;
    fn has_multiple_rows(&mut self) -> Result<bool, DeserError> {
        Ok(self.rows.len() > 1_usize)
    }

    fn last_row(&self) -> Option<&<Self as DeserializableResultset>::ROW> {
        self.rows.last()
    }

    fn last_row_mut(&mut self) -> Option<&mut <Self as DeserializableResultset>::ROW> {
        self.rows.last_mut()
    }

    fn reverse_rows(&mut self) {
        self.rows.reverse()
    }

    fn pop_row(&mut self) -> Result<Option<<Self as DeserializableResultset>::ROW>, DeserError> {
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

impl fmt::Debug for MockResultset {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "Some MockResultSet")
    }
}


impl IntoIterator for MockResultset {
    type Item = Row<MockMetadata, MockValue>;
    type IntoIter = <Vec<Row<MockMetadata, MockValue>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.rows.into_iter()
    }
}
