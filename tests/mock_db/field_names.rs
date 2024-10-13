// Metadata for the fields in a result row
#[derive(Debug)]
pub struct FieldNames {
    fields: Vec<String>,
}
impl FieldNames {
    pub fn new(fields: &[&'static str]) -> FieldNames {
        FieldNames {
            fields: fields.iter().map(|s| String::from(*s)).collect(),
        }
    }

    pub fn number_of_fields(&self) -> usize {
        self.fields.len()
    }

    pub fn field_name(&self, i: usize) -> Option<&str> {
        self.fields.get(i).map(String::as_str)
    }
}
