use serde;
use std::error;
use std::error::Error;
use std::fmt;

use de::ConversionError;

/// The errors that can arise while deserializing witrh serde_db::de.
pub enum DeserError {
    /// Deserialization failed due to a conversion error.
    ConversionError(ConversionError),
    /// Raised when there is a general error in the serde framework when deserializing.
    SerdeError(String),
    /// Structure of target object does not fit to the structure of the object being deserialized
    Implementation(String),
    ///
    NotImplemented(&'static str),
    ///
    UnknownField(String),
    ///
    MissingField(String),
    ///
    TrailingRows,
    ///
    TrailingCols,
}

impl error::Error for DeserError {
    fn description(&self) -> &str {
        match *self {
            DeserError::ConversionError(_) => "Conversion of database type to rust type failed",
            DeserError::SerdeError(_) => "general error from the deserialization framework",
            DeserError::Implementation(_) => "error in the implementation of the serde_db",
            DeserError::NotImplemented(_) => "function not implemented",
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

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            DeserError::ConversionError(ref e) => Some(e),
            _ => None,
        }
    }
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
            DeserError::Implementation(ref s) |
            DeserError::UnknownField(ref s) => {
                write!(formatter, "{} (\"{}\")", self.description(), s)
            }
            DeserError::NotImplemented(s) => {
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
            DeserError::Implementation(ref s) |
            DeserError::UnknownField(ref s) |
            DeserError::MissingField(ref s) => write!(fmt, "{} ", s),
            DeserError::NotImplemented(ref s) => write!(fmt, "{} ", s),
            DeserError::TrailingRows => write!(fmt, "{} ", "TrailingRows"),
            DeserError::TrailingCols => write!(fmt, "{} ", "TrailingCols"),
        }
    }
}
/// Shortcut
pub type DeserResult<T> = Result<T, DeserError>;
