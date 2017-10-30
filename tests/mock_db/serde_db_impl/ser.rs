use chrono::NaiveDateTime;
use std::str::FromStr;
use std::error::Error;

use mock_db;
use mock_db::MValue;
use mock_db::ParameterType;
use serde_db::ser::{DbvFactory, SerializationError};
use std::{i16, i32, i8, u16, u32, u8};

impl DbvFactory for ParameterType {
    type DBV = MValue;

    fn from_bool(&self, _value: bool) -> Result<Self::DBV, SerializationError> {
        Err(SerializationError::TypeMismatch("bool", self.descriptor()))
    }
    fn from_i8(&self, value: i8) -> Result<Self::DBV, SerializationError> {
        match *self {
            ParameterType::Short => Ok(MValue::Short(i16::from(value))),
            ParameterType::NullableShort => Ok(MValue::NullableShort(Some(i16::from(value)))),
            _ => Err(SerializationError::TypeMismatch("i8", self.descriptor())),
        }
    }
    fn from_i16(&self, value: i16) -> Result<Self::DBV, SerializationError> {
        match *self {
            ParameterType::Short => Ok(MValue::Short(value)),
            ParameterType::NullableShort => Ok(MValue::NullableShort(Some(value))),
            _ => Err(SerializationError::TypeMismatch("i16", self.descriptor())),
        }
    }
    fn from_i32(&self, value: i32) -> Result<Self::DBV, SerializationError> {
        match *self {
            ParameterType::Short => {
                if (value >= i32::from(i16::MIN)) && (value <= i32::from(i16::MAX)) {
                    Ok(MValue::Short(value as i16))
                } else {
                    Err(SerializationError::RangeErr("i32", self.descriptor()))
                }
            }
            ParameterType::NullableShort => {
                if (value >= i32::from(i16::MIN)) && (value <= i32::from(i16::MAX)) {
                    Ok(MValue::NullableShort(Some(value as i16)))
                } else {
                    Err(SerializationError::RangeErr("i32", self.descriptor()))
                }
            }
            _ => Err(SerializationError::TypeMismatch("i32", self.descriptor())),
        }
    }
    fn from_i64(&self, value: i64) -> Result<Self::DBV, SerializationError> {
        match *self {
            ParameterType::Short => {
                if (value >= i64::from(i16::MIN)) && (value <= i64::from(i16::MAX)) {
                    Ok(MValue::Short(value as i16))
                } else {
                    Err(SerializationError::RangeErr("i64", self.descriptor()))
                }
            }
            ParameterType::NullableShort => {
                if (value >= i64::from(i16::MIN)) && (value <= i64::from(i16::MAX)) {
                    Ok(MValue::NullableShort(Some(value as i16)))
                } else {
                    Err(SerializationError::RangeErr("i64", self.descriptor()))
                }
            }
            _ => Err(SerializationError::TypeMismatch("i64", self.descriptor())),
        }
    }
    fn from_u8(&self, _value: u8) -> Result<Self::DBV, SerializationError> {
        Err(SerializationError::TypeMismatch("u8", self.descriptor()))
    }
    fn from_u16(&self, _value: u16) -> Result<Self::DBV, SerializationError> {
        Err(SerializationError::TypeMismatch("u16", self.descriptor()))
    }
    fn from_u32(&self, _value: u32) -> Result<Self::DBV, SerializationError> {
        Err(SerializationError::TypeMismatch("u32", self.descriptor()))
    }
    fn from_u64(&self, _value: u64) -> Result<Self::DBV, SerializationError> {
        Err(SerializationError::TypeMismatch("u64", self.descriptor()))
    }
    fn from_f32(&self, _value: f32) -> Result<Self::DBV, SerializationError> {
        Err(SerializationError::TypeMismatch("f32", self.descriptor()))
    }
    fn from_f64(&self, _value: f64) -> Result<Self::DBV, SerializationError> {
        Err(SerializationError::TypeMismatch("f64", self.descriptor()))
    }
    fn from_char(&self, _value: char) -> Result<Self::DBV, SerializationError> {
        Err(SerializationError::TypeMismatch("char", self.descriptor()))
    }
    fn from_str(&self, value: &str) -> Result<Self::DBV, SerializationError> {
        match *self {
            ParameterType::String => Ok(MValue::String(value.to_owned())),
            ParameterType::NullableString => Ok(MValue::NullableString(Some(value.to_owned()))),
            ParameterType::Timestamp => Ok(MValue::Timestamp(mock_db_timestamp(value)?)),
            ParameterType::NullableTimestamp => {
                Ok(MValue::NullableTimestamp(Some(mock_db_timestamp(value)?)))
            }
            _ => Err(SerializationError::TypeMismatch("str", self.descriptor())),
        }
    }
    fn from_bytes(&self, _value: &[u8]) -> Result<Self::DBV, SerializationError> {
        Err(SerializationError::TypeMismatch("bytes", self.descriptor()))
    }
    fn from_none(&self) -> Result<Self::DBV, SerializationError> {
        match *self {
            ParameterType::NullableShort => Ok(MValue::NullableShort(None)),
            ParameterType::NullableString => Ok(MValue::NullableString(None)),
            ParameterType::NullableTimestamp => Ok(MValue::NullableTimestamp(None)),
            _ => Err(SerializationError::TypeMismatch("none", self.descriptor())),
        }
    }
    fn descriptor(&self) -> String {
        match *self {
            ParameterType::Short => "Short",
            ParameterType::NullableShort => "NullableShort",
            ParameterType::String => "String",
            ParameterType::NullableString => "NullableString",
            ParameterType::Timestamp => "Timestamp",
            ParameterType::NullableTimestamp => "NullableTimestamp",
        }.to_string()
    }
}

fn mock_db_timestamp(value: &str) -> Result<mock_db::Timestamp, SerializationError> {
    use serde::ser::Error;
    match NaiveDateTime::from_str(value) {
        Ok(ts) => Ok(mock_db::Timestamp(ts)),
        Err(e) => Err(SerializationError::custom(e.description())),
    }
}


impl From<SerializationError> for mock_db::Error {
    fn from(e: SerializationError) -> mock_db::Error {
        mock_db::Error::SERIALIZATION(e)
    }
}
