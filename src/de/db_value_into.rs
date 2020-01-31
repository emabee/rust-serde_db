use crate::de::conversion_error::ConversionError;

/// Converts the given database value into a specific rust type.
///
/// We recommend to implement `try_into` as gracefully as possible, i.e.,
/// supporting as many conversions as possible. For the numeric
/// types this requires some lines of code, but the effort pays off for the users.
///
/// Example:
///
/// ```rust,ignore
/// impl DbValueInto<u32> for MyDbValue {
///     fn try_into(self) -> Result<u32, ConversionError> {
///         match self {
///             MyDbValue::TINYINT(u) => Ok(u as u32),
///
///             MyDbValue::SMALLINT(i) => if i >= 0 {
///                 Ok(i as u32)
///             } else {
///                 Err(ConversionError::NumberRange(...))
///             }
///
///             MyDbValue::INT(i) => if i >= 0 {
///                 Ok(i as u32)
///             } else {
///                 Err(ConversionError::NumberRange(...))
///             }
///
///             MyDbValue::BIGINT(i) => if (i >= 0) && (i <= u32::MAX as i64) {
///                 Ok(i as u32)
///             } else {
///                 Err(ConversionError::NumberRange(...))
///             }
///
///             _ => Err(ConversionError::ValueType(...)),
///         }
///     }
///  }
/// ```
pub trait DbValueInto<T> {
    /// Converts the database value into the target rust type.
    ///
    /// # Errors
    ///
    /// `ConversionError` if the value cannot be converted into the target type.
    fn try_into(self) -> Result<T, ConversionError>;
}
