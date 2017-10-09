use serde_db::de::DeserializationError;
use std::error;
use std::fmt;

pub type MockResult<T> = Result<T, MockError>;

#[derive(Debug)]
pub enum MockError {
    DESERIALIZATION(DeserializationError),
}

impl error::Error for MockError {
    fn description(&self) -> &str {
        match *self {
            MockError::DESERIALIZATION(ref e) => e.description(),
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            MockError::DESERIALIZATION(ref e) => Some(e),
        }
    }
}

impl fmt::Display for MockError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MockError::DESERIALIZATION(ref e) => write!(fmt, "{}", e),
        }
    }
}

impl From<DeserializationError> for MockError {
    fn from(e: DeserializationError) -> MockError {
        MockError::DESERIALIZATION(e)
    }
}
