use chrono::{Datelike, NaiveDateTime, Timelike};
use serde;
use serde_db::de::DbValue;
use std::fmt;

use crate::mock_db;

#[derive(Clone, Debug, PartialEq)]
pub struct Timestamp(pub NaiveDateTime);

impl fmt::Display for Timestamp {
    // The format chosen supports the derserialization to chrono types.
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:07}",
            self.0.year(),
            self.0.month(),
            self.0.day(),
            self.0.hour(),
            self.0.minute(),
            self.0.second(),
            self.0.nanosecond() * 100
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MValue {
    Null,
    Short(i16),
    Double(f64),
    String(String),
    Timestamp(Timestamp),
}

impl MValue {
    pub fn new_null() -> MValue {
        MValue::Null
    }
    pub fn new_short(i: i16) -> MValue {
        MValue::Short(i)
    }
    pub fn new_double(f: f64) -> MValue {
        MValue::Double(f)
    }
    pub fn new_string(s: String) -> MValue {
        MValue::String(s)
    }
    pub fn new_ts(ts: NaiveDateTime) -> MValue {
        MValue::Timestamp(Timestamp(ts))
    }

    pub fn try_into<'de, T>(self) -> mock_db::Result<T>
    where
        T: serde::de::Deserialize<'de>,
    {
        trace!("MValue::try_into()");
        Ok(DbValue::into_typed(self)?)
    }
}
