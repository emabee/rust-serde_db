use serde;
use std::marker::Sized;
use std::{i16, i32, i8, u16, u32, u8};

use crate::de::field_deserializer::FieldDeserializer;
use crate::de::{ConversionError, DeserializationError};

/// Defines into which rust types we support deserialization of fields.
pub trait DbValue:
    Sized
    + DbValueInto<bool>
    + DbValueInto<u8>
    + DbValueInto<u16>
    + DbValueInto<u32>
    + DbValueInto<u64>
    + DbValueInto<i8>
    + DbValueInto<i16>
    + DbValueInto<i32>
    + DbValueInto<i64>
    + DbValueInto<f32>
    + DbValueInto<f64>
    + DbValueInto<String>
    + DbValueInto<Vec<u8>>
{
    /// Returns true if this is a NULL value.
    fn is_null(&self) -> bool;

    /// Converts the DbValue into a plain rust value.
    fn into_typed<'de, T>(self) -> Result<T, DeserializationError>
    where
        T: serde::de::Deserialize<'de>,
    {
        Ok(serde::de::Deserialize::deserialize(
            FieldDeserializer::new(self),
        )?)
    }
}

/// Conversion into a specific type.
///
/// We recommend to implement this function in an as-graceful-as-possible mode, i.e.,
/// supporting every call as long as the concrete value can be converted. For the numeric
/// types this requires quite some lines of code, but the effort pays off: it makes the usage of
/// `serde_db` much more user-friendly.
///
/// Example:
///
/// ```ignore
/// impl DbValueInto<u32> for MyDbValue {
///     fn try_into(self) -> Result<u32, ConversionError> {
///         match self {
///             MyDbValue::TINYINT(u) |
///             MyDbValue::NULLABLE_TINYINT(Some(u)) => Ok(u as u32),
///
///             MyDbValue::SMALLINT(i) |
///             MyDbValue::NULLABLE_SMALLINT(Some(i)) => {
///                 if i >= 0 {
///                     Ok(i as u32)
///                 } else {
///                     Err(ConversionError::NumberRange(...))
///                 }
///             }
///
///             MyDbValue::INT(i) |
///             MyDbValue::NULLABLE_INT(Some(i)) => {
///                 if i >= 0 {
///                     Ok(i as u32)
///                 } else {
///                     Err(ConversionError::NumberRange(...))
///                 }
///             }
///
///             MyDbValue::BIGINT(i) |
///             MyDbValue::NULLABLE_BIGINT(Some(i)) => {
///                 if (i >= 0) && (i <= u32::MAX as i64) {
///                     Ok(i as u32)
///                 } else {
///                     Err(ConversionError::NumberRange(...))
///                 }
///             }
///
///             _ => Err(ConversionError::ValueType(...)),
///         }
///     }
///  }
/// ```
pub trait DbValueInto<T> {
    /// Tries to convert into type T.
    fn try_into(self) -> Result<T, ConversionError>;
}
