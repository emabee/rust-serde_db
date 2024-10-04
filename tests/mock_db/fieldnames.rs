// Metadata for the fields in a result row
#[derive(Debug)]
pub struct Fieldnames {
    fields: Vec<String>,
}
impl Fieldnames {
    pub fn new(fields: &[&'static str]) -> Fieldnames {
        Fieldnames {
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
