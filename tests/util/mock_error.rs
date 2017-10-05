use serde_db::de::DeserError;

#[derive(Debug)]
pub enum MockError {
    DESERIALIZATION(DeserError),
}

pub type MockResult<T> = Result<T, MockError>;

impl From<DeserError> for MockError {
    fn from(e: DeserError) -> MockError {
        MockError::DESERIALIZATION(e)
    }
}
