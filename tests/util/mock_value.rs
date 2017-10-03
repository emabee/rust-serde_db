use chrono::{NaiveDateTime, Datelike, Timelike};
use std::fmt;
use serde;
use serde_db::de::{ConversionError, DbValue, DbValueInto};
use super::mock_error::MockError;

#[derive(Clone,Debug)]
pub struct MockTimestamp(pub NaiveDateTime);

impl fmt::Display for MockTimestamp {
    // The format chosen supports the conversion to chrono types.
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt,
               "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:07}",
               self.0.year(),
               self.0.month(),
               self.0.day(),
               self.0.hour(),
               self.0.minute(),
               self.0.second(),
               self.0.nanosecond() * 100)
    }
}




#[derive(Clone, Debug)]
pub enum MockValue {
    SHORT(i16),
    STRING(String),
    TIMESTAMP(MockTimestamp),
}

impl MockValue {
    pub fn new_short(i: i16) -> MockValue {
        MockValue::SHORT(i)
    }
    pub fn new_string(s: String) -> MockValue {
        MockValue::STRING(s)
    }
    pub fn new_ts(ts: NaiveDateTime) -> MockValue {
        MockValue::TIMESTAMP(MockTimestamp(ts))
    }

    /// Converts the DbValue into a plain rust value.
    pub fn into_typed<'de, T>(self) -> Result<T, MockError>
        where T: serde::de::Deserialize<'de>
    {
        trace!("MockValue::into_typed()");
        Ok(DbValue::into_typed(self)?)
        // Ok(serde::de::Deserialize::deserialize(FieldDeserializer::new(self))?)
    }
}

impl DbValue for MockValue {
    fn is_null(&self) -> bool {
        false
    }
}

impl DbValueInto<bool> for MockValue {
    fn try_into(self) -> Result<bool, ConversionError> {
        panic!("DbValueInto<bool> not implemented")
    }
}
impl DbValueInto<u8> for MockValue {
    fn try_into(self) -> Result<u8, ConversionError> {
        panic!("DbValueInto<u8> not implemented")
    }
}
impl DbValueInto<u16> for MockValue {
    fn try_into(self) -> Result<u16, ConversionError> {
        panic!("DbValueInto<u16> not implemented")
    }
}
impl DbValueInto<u32> for MockValue {
    fn try_into(self) -> Result<u32, ConversionError> {
        panic!("DbValueInto<u32> not implemented")
    }
}
impl DbValueInto<u64> for MockValue {
    fn try_into(self) -> Result<u64, ConversionError> {
        panic!("DbValueInto<u64> not implemented")
    }
}
impl DbValueInto<i8> for MockValue {
    fn try_into(self) -> Result<i8, ConversionError> {
        panic!("DbValueInto<i8> not implemented")
    }
}
impl DbValueInto<i16> for MockValue {
    fn try_into(self) -> Result<i16, ConversionError> {
        panic!("DbValueInto<i16> not implemented")
    }
}
impl DbValueInto<i32> for MockValue {
    fn try_into(self) -> Result<i32, ConversionError> {
        match self {
            MockValue::SHORT(i) => Ok(i as i32),
            mv => panic!("DbValueInto<i32> not implemented for {:?}", mv),
        }
    }
}
impl DbValueInto<i64> for MockValue {
    fn try_into(self) -> Result<i64, ConversionError> {
        panic!("DbValueInto<i64> not implemented")
    }
}
impl DbValueInto<f32> for MockValue {
    fn try_into(self) -> Result<f32, ConversionError> {
        panic!("DbValueInto<f32> not implemented")
    }
}
impl DbValueInto<f64> for MockValue {
    fn try_into(self) -> Result<f64, ConversionError> {
        panic!("DbValueInto<f64> not implemented")
    }
}
impl DbValueInto<String> for MockValue {
    fn try_into(self) -> Result<String, ConversionError> {
        trace!("try_into -> String");
        match self {
            MockValue::STRING(s) => Ok(s),
            MockValue::TIMESTAMP(ts) => Ok(ts.to_string()),
            mv => panic!("DbValueInto<String> not implemented for {:?}", mv),
        }
    }
}
impl DbValueInto<NaiveDateTime> for MockValue {
    fn try_into(self) -> Result<NaiveDateTime, ConversionError> {
        trace!("try_into -> NaiveDateTime");
        match self {
            MockValue::TIMESTAMP(MockTimestamp(ts)) => Ok(ts),
            mv => panic!("DbValueInto<NaiveDateTime> not implemented for {:?}", mv),
        }
    }
}
impl DbValueInto<Vec<u8>> for MockValue {
    fn try_into(self) -> Result<Vec<u8>, ConversionError> {
        panic!("DbValueInto<Vec<u8>> not implemented")
    }
}
