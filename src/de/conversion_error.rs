use std::error::Error;
use std::fmt;

/// An error type for implementors of DbValue.
pub enum ConversionError {
    /// The DbValue cannot be converted into the desired rust type.
    ValueType(String),

    /// The DbValue is to big or too small (negative) fo conversion into the desired rust type.
    NumberRange(String),

    /// The DbValue was not yet completely loaded, and further loading is not possible anymore.
    Incomplete(String),
}

impl Error for ConversionError {
    fn description(&self) -> &str {
        match *self {
            ConversionError::ValueType(_) => "value types do not match",
            ConversionError::NumberRange(_) => "number range exceeded",
            ConversionError::Incomplete(_) => "incomplete LOB",
        }
    }
}

impl fmt::Debug for ConversionError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConversionError::ValueType(ref s) |
            ConversionError::NumberRange(ref s) |
            ConversionError::Incomplete(ref s) => {
                write!(formatter, "{}: (\"{}\")", self.description(), s)
            }
        }
    }
}

impl fmt::Display for ConversionError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConversionError::ValueType(ref s) |
            ConversionError::NumberRange(ref s) |
            ConversionError::Incomplete(ref s) => write!(fmt, "{} ", s),
        }
    }
}
