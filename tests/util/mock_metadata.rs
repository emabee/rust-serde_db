use serde_db::de::row::Metadata;
use std::fmt;

#[derive(Debug)]
pub struct MockMetadata {
    fields: Vec<String>,
}
impl MockMetadata {
    pub fn new(fields: Vec<&'static str>) -> MockMetadata {
        MockMetadata { fields: fields.iter().map(|s| String::from(*s)).collect() }
    }
}
impl Metadata for MockMetadata {
    fn number_of_fields(&self) -> usize {
        self.fields.len()
    }

    fn get_fieldname(&self, i: usize) -> Option<&String> {
        self.fields.get(i)
    }
}

impl fmt::Display for MockMetadata {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "Some MockMetadata")?;
        // for field_metadata in &self.fields {
        //     match self.names.get(field_metadata.column_displayname as usize) {
        //         Some(fieldname) => write!(fmt, "{}, ", fieldname).unwrap(),
        //         None => write!(fmt, "<unnamed>, ").unwrap(),
        //     };
        // }
        Ok(())
    }
}
