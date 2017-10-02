use serde_db::de::{ConversionError, DeserError};

#[derive(Debug)]
pub enum MockError {
    MESSAGE(String),
    DESERIALIZATION(DeserError),
}

impl From<DeserError> for MockError {
    fn from(e: DeserError) -> MockError {
        MockError::DESERIALIZATION(e)
    }
}

// FIXME try to avoid this!!
impl From<ConversionError> for MockError {
    fn from(e: ConversionError) -> MockError {
        MockError::DESERIALIZATION(DeserError::ConversionError(e))
    }
}

pub type MockResult<T> = Result<T, MockError>;
