use thiserror::Error;

/// An error type for implementors of `DbValue`.
#[derive(Debug, Error)]
pub enum ConversionError {
    /// The DbValue cannot be converted into the desired rust type.
    #[error(
        "The DbValue cannot be converted into the desired rust type: value types do not match"
    )]
    ValueType(String),

    /// The DbValue is to big or too small (negative) for conversion into the desired rust type.
    #[error(
        "The DbValue is too big or too small for the desired rust type: number range exceeded"
    )]
    NumberRange(String),

    /// The DbValue was not yet completely loaded, and further loading is not possible anymore.
    #[error(
        "The DbValue was not yet completely loaded, and further loading is not possible anymore"
    )]
    Incomplete(String),
}
