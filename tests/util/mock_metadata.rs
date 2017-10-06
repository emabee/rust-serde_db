use serde_db::de::row::Metadata;

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
