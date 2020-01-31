use serde;
use serde_db::de::DeserializableResultset;
use std::rc::Rc;

use crate::mock_db;

// Simple ResultSet for test purposes.
// Rows are added using push().
#[derive(Debug)]
pub struct Resultset {
    next_rows: Vec<mock_db::Row>,
    row_iter: <Vec<mock_db::Row> as IntoIterator>::IntoIter,
    md: Rc<mock_db::Fieldnames>,
}
impl Resultset {
    pub fn new(fields: &[&'static str]) -> Resultset {
        Resultset {
            next_rows: Vec::<mock_db::Row>::new(),
            row_iter: Vec::<mock_db::Row>::new().into_iter(),
            md: Rc::new(mock_db::Fieldnames::new(fields)),
        }
    }

    pub fn push(&mut self, values: Vec<mock_db::MValue>) {
        assert_eq!(self.md.number_of_fields(), values.len());
        self.next_rows
            .push(mock_db::Row::new(Rc::clone(&self.md), values))
    }

    pub fn next(&mut self) -> Option<mock_db::Row> {
        match self.row_iter.next() {
            Some(r) => Some(r),
            None => {
                let mut tmp_vec = Vec::<mock_db::Row>::new();
                std::mem::swap(&mut tmp_vec, &mut self.next_rows);
                self.row_iter = tmp_vec.into_iter();
                self.row_iter.next()
            }
        }
    }

    pub fn len(&self) -> usize {
        self.next_rows.len() + self.row_iter.as_slice().len()
    }

    pub fn has_multiple_rows(&mut self) -> bool {
        self.len() > 1
    }

    pub fn number_of_fields(&self) -> usize {
        self.md.number_of_fields()
    }

    pub fn fieldname(&self, i: usize) -> Option<&str> {
        self.md.fieldname(i)
    }

    // Expose the capability from serde_db: see module serde_db_impl for more...
    pub fn try_into<'de, T>(self) -> mock_db::Result<T>
    where
        T: serde::de::Deserialize<'de>,
    {
        trace!("Resultset::try_into()");
        Ok(DeserializableResultset::into_typed(self)?)
    }
}

impl Iterator for Resultset {
    type Item = mock_db::Row;
    fn next(&mut self) -> Option<mock_db::Row> {
        self.next()
    }
}
