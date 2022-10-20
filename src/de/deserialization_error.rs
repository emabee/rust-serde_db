use crate::de::ConversionError;
use thiserror::Error;

/// The errors that can arise while deserializing with `serde_db::de`.
#[derive(Error)]
pub enum DeserializationError {
    /// Deserialization failed due to a conversion error.
    #[error("Deserialization failed due to a conversion error")]
    ConversionError(#[from] ConversionError),

    /// Error reported from serde framework.
    #[error("serde error")]
    SerdeError(String),

    /// The structure of the target object does not fit to the structure of the object being
    /// deserialized.
    #[error("incompatible target structure")]
    Usage(String),

    /// Thrown by functions in the Deserializer interface that are not implemented.
    /// This exception should never be seen in practice.
    #[error("this exception should never be seen in practice")]
    NotImplemented(&'static str),

    /// The target structure misses a field for which data are provided.
    #[error("the deserialization target misses a field for which data are provided")]
    UnknownField(String),

    /// The deserialization cannot consume all existing rows.
    #[error("cannot consume all existing rows")]
    TrailingRows,

    /// The deserialization cannot consume all existing columns.
    #[error("cannot consume all existing columns")]
    TrailingCols,
}

impl serde::de::Error for DeserializationError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        DeserializationError::SerdeError(msg.to_string())
    }
}

impl std::fmt::Debug for DeserializationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::ConversionError(ref e) => write!(formatter, "{e:?}"),
            Self::NotImplemented(s) => write!(formatter, "{self}: {s}"),
            Self::SerdeError(ref s) | Self::UnknownField(ref s) | Self::Usage(ref s) => {
                write!(formatter, "{self}: {s}")
            }
            Self::TrailingRows | Self::TrailingCols => write!(formatter, "{self}"),
        }
    }
}

/// A specialized Result type for deserialization.
pub type DeserializationResult<T> = Result<T, DeserializationError>;
