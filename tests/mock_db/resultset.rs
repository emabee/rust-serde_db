use serde;
use serde_db::de::DeserializableResultset;
use std::rc::Rc;

use mock_db;

pub struct Resultset {
    pub rows: Vec<mock_db::Row>,
    pub md: Rc<mock_db::Fieldnames>,
}
impl Resultset {
    pub fn new(fields: &[&'static str]) -> Resultset {
        Resultset {
            rows: Vec::<mock_db::Row>::new(),
            md: Rc::new(mock_db::Fieldnames::new(fields)),
        }
    }
    pub fn len(&self) -> usize {
        self.rows.len()
    }
    pub fn push(&mut self, values: Vec<mock_db::MValue>) {
        assert_eq!(self.md.number_of_fields(), values.len());
        self.rows
            .push(mock_db::Row::new(Rc::clone(&self.md), values))
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

impl IntoIterator for Resultset {
    type Item = mock_db::Row;
    type IntoIter = <Vec<mock_db::Row> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.rows.into_iter()
    }
}
