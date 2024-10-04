use crate::mock_db;
use crate::mock_db::{MValue, Resultset};
use serde_db::de::{
    ConversionError, DbValue, DbValueInto, DeserializableResultset, DeserializationError,
};
use std::{i16, i32, i8, u16, u32, u8};

fn not_implemented(s: &'static str) -> ConversionError {
    ConversionError::ValueType(format!("{} not implemented", s))
}

impl DbValue for MValue {
    fn is_null(&self) -> bool {
        if let MValue::Null = *self {
            true
        } else {
            false
        }
    }
}

impl DbValueInto<bool> for MValue {
    fn try_into(self) -> Result<bool, ConversionError> {
        Err(not_implemented("DbValueInto<bool>"))
    }
}
impl DbValueInto<u8> for MValue {
    fn try_into(self) -> Result<u8, ConversionError> {
        Err(not_implemented("DbValueInto<u8>"))
    }
}
impl DbValueInto<u16> for MValue {
    fn try_into(self) -> Result<u16, ConversionError> {
        Err(not_implemented("DbValueInto<u16>"))
    }
}
impl DbValueInto<u32> for MValue {
    fn try_into(self) -> Result<u32, ConversionError> {
        Err(not_implemented("DbValueInto<u32>"))
    }
}
impl DbValueInto<u64> for MValue {
    fn try_into(self) -> Result<u64, ConversionError> {
        Err(not_implemented("DbValueInto<u64>"))
    }
}
impl DbValueInto<i8> for MValue {
    fn try_into(self) -> Result<i8, ConversionError> {
        Err(not_implemented("DbValueInto<i8>"))
    }
}
impl DbValueInto<i16> for MValue {
    fn try_into(self) -> Result<i16, ConversionError> {
        match self {
            MValue::Short(i) => Ok(i),
            mv => Err(ConversionError::ValueType(format!(
                "DbValueInto<i16> not implemented for {:?}",
                mv
            ))),
        }
    }
}
impl DbValueInto<i32> for MValue {
    fn try_into(self) -> Result<i32, ConversionError> {
        match self {
            MValue::Short(i) => Ok(i32::from(i)),
            mv => Err(ConversionError::ValueType(format!(
                "DbValueInto<i32> not implemented for {:?}",
                mv
            ))),
        }
    }
}
impl DbValueInto<i64> for MValue {
    fn try_into(self) -> Result<i64, ConversionError> {
        Err(not_implemented("DbValueInto<i64>"))
    }
}
impl DbValueInto<f32> for MValue {
    fn try_into(self) -> Result<f32, ConversionError> {
        Err(not_implemented("DbValueInto<f32>"))
    }
}
impl DbValueInto<f64> for MValue {
    fn try_into(self) -> Result<f64, ConversionError> {
        match self {
            MValue::Double(f) => Ok(f),
            mv => Err(ConversionError::ValueType(format!(
                "DbValueInto<f64> not implemented for {:?}",
                mv
            ))),
        }
    }
}
impl DbValueInto<String> for MValue {
    fn try_into(self) -> Result<String, ConversionError> {
        trace!("try_into -> String");
        match self {
            MValue::String(s) => Ok(s),
            MValue::Timestamp(ts) => Ok(ts.to_string()),
            MValue::Double(f) => Ok(f.to_string()),
            mv => Err(ConversionError::ValueType(format!(
                "DbValueInto<String> not implemented for {:?}",
                mv
            ))),
        }
    }
}
impl DbValueInto<Vec<u8>> for MValue {
    fn try_into(self) -> Result<Vec<u8>, ConversionError> {
        Err(not_implemented("DbValueInto<Vec<u8>>"))
    }
}

impl DeserializableResultset for Resultset {
    type E = mock_db::Error;
    type Row = mock_db::Row;

    fn has_multiple_rows(&mut self) -> Result<bool, DeserializationError> {
        Ok(self.has_multiple_rows())
    }

    fn next(&mut self) -> Result<Option<mock_db::Row>, DeserializationError> {
        Ok(self.next())
    }

    fn number_of_fields(&self) -> usize {
        self.number_of_fields()
    }

    fn fieldname(&self, i: usize) -> Option<&str> {
        self.fieldname(i)
    }
}

impl From<DeserializationError> for mock_db::Error {
    fn from(e: DeserializationError) -> mock_db::Error {
        mock_db::Error::DESERIALIZATION(e)
    }
}
