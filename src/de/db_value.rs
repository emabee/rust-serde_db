use serde;
use std::{u8, u16, u32, i8, i16, i32};
use std::marker::Sized;

use super::conversion_error::ConversionError;
use super::deserialization_error::DeserError;
use super::field_deserializer::FieldDeserializer;

/// Defines into which rust types we support deserialization of fields.
/// FIXME Add more info + example how to implement i32
pub trait DbValue:
    Clone
    + Sized
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
    fn into_typed<'de, T>(self) -> Result<T, DeserError>
        where T: serde::de::Deserialize<'de>
    {
        trace!("DbValue::into_typed()");
        Ok(serde::de::Deserialize::deserialize(FieldDeserializer::new(self))?)
    }
}


/// Conversion into a specific type.
pub trait DbValueInto<T> {
    /// Tries to convert into type T.
    fn try_into(self) -> Result<T, ConversionError>;
}
