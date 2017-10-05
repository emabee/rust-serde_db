use std::error::Error;
use serde;
use std::fmt;
use de::conversion_error::ConversionError;

/// The errors that can arise while deserializing a ResultSet into a standard rust type/struct/Vec
pub enum DeserError {
    /// Deserialization failed due to a conversion error.
    ConversionError(ConversionError),
    /// Raised when there is a general error in the serde framework when deserializing a type.
    SerdeError(String),
    /// Structure of target object does not fit to the structure of the resultset or row being deserialized
    BadStructure(String),
    ///
    UnknownField(String),
    ///
    MissingField(String),
    ///
    TrailingRows,
    ///
    TrailingCols,
}
impl Error for DeserError {
    fn description(&self) -> &str {
        match *self {
            DeserError::ConversionError(_) => "Conversion of database type to rust type failed",
            DeserError::SerdeError(_) => "general error from the deserialization framework",
            DeserError::BadStructure(_) => {
                "error in the implementation of the resultset deserialization"
            }
            DeserError::UnknownField(_) => {
                "the target structure misses a field for which data are provided"
            }
            DeserError::MissingField(_) => {
                "the target structure contains a field for which no data are provided"
            }
            DeserError::TrailingRows => "trailing rows",
            DeserError::TrailingCols => "trailing columns",
        }
    }
}

pub fn prog_err(s: &str) -> DeserError {
    DeserError::BadStructure(String::from(s))
}

impl From<ConversionError> for DeserError {
    fn from(error: ConversionError) -> DeserError {
        DeserError::ConversionError(error)
    }
}


impl serde::de::Error for DeserError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        DeserError::SerdeError(msg.to_string())
    }
}

impl fmt::Debug for DeserError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeserError::ConversionError(ref e) => write!(formatter, "{:?}", e),
            DeserError::SerdeError(ref s) |
            DeserError::BadStructure(ref s) |
            DeserError::UnknownField(ref s) => {
                write!(formatter, "{} (\"{}\")", self.description(), s)
            }
            DeserError::MissingField(ref s) => {
                write!(formatter,
                       "{} (\"{}\"); note that the field mapping is case-sensitive, and partial \
                        deserialization is not supported",
                       self.description(),
                       s)
            }
            DeserError::TrailingRows | DeserError::TrailingCols => {
                write!(formatter, "{}", self.description())
            }
        }
    }
}
impl fmt::Display for DeserError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeserError::ConversionError(ref e) => write!(fmt, "{}", e),
            DeserError::SerdeError(ref s) |
            DeserError::BadStructure(ref s) |
            DeserError::UnknownField(ref s) |
            DeserError::MissingField(ref s) => write!(fmt, "{} ", s),
            DeserError::TrailingRows => write!(fmt, "{} ", "TrailingRows"),
            DeserError::TrailingCols => write!(fmt, "{} ", "TrailingCols"),
        }
    }
}
/// Shortcut
pub type DeserResult<T> = Result<T, DeserError>;
