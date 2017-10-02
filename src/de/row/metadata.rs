use std::fmt;

/// A simple view on the metadata.
pub trait Metadata: fmt::Display + fmt::Debug {
    /// Returns the number of fields in each row
    fn number_of_fields(&self) -> usize;

    /// Returns the name of the n_th field (zero-based: the index of the first field is 0).
    fn get_fieldname(&self, field_idx: usize) -> Option<&String>;
}
