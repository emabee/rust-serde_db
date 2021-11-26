#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

mod mock_db;
mod util;

use crate::mock_db::{MValue, ParameterType as PT};
use chrono::{NaiveDate, NaiveDateTime};
use flexi_logger::LoggerHandle;

#[test] // cargo test --test test_serialization -- --nocapture
pub fn test_serialization() {
    let mut loghandle = util::init_logger();

    match impl_test_serialization(&mut loghandle) {
        Err(e) => {
            error!("test_serialization() failed with {:?}", e);
            assert!(false)
        }
        Ok(_) => debug!("test_serialization() ended successful"),
    }
}

fn impl_test_serialization(loghandle: &mut LoggerHandle) -> mock_db::Result<()> {
    tuple_serialization(loghandle)?;
    struct_serialization(loghandle)?;
    Ok(())
}

fn tuple_serialization(_loghandle: &mut LoggerHandle) -> mock_db::Result<()> {
    info!(
        "Test tuple with all conversions (plain -> plain, Option -> plain, plain -> Option, \
         Option -> Option)"
    );
    let input_metadata: Vec<PT> = vec![
        // int
        PT::Short,
        PT::Short,
        PT::NullableShort,
        PT::NullableShort,
        PT::NullableShort,
        // string
        PT::String,
        PT::String,
        PT::NullableString,
        PT::NullableString,
        PT::NullableString,
        // timestamp
        PT::Timestamp,
        PT::Timestamp,
        PT::NullableTimestamp,
        PT::NullableTimestamp,
        PT::NullableTimestamp,
    ];
    let i_none: Option<i32> = None;
    let s_none: Option<String> = None;
    let t_none: Option<NaiveDateTime> = None;

    let input = (
        // int
        0_i32,
        Some(1_i32),
        2_i32,
        Some(3_i32),
        i_none,
        // string
        "Five".to_string(),
        Some("Six".to_string()),
        "Seven".to_string(),
        Some("Eight".to_string()),
        s_none,
        // timestamp
        NaiveDate::from_ymd(2011, 1, 1).and_hms_nano(1, 1, 1, 100_000_000),
        Some(NaiveDate::from_ymd(2012, 2, 2).and_hms_nano(2, 2, 2, 200_000_000)),
        NaiveDate::from_ymd(2013, 3, 3).and_hms_nano(3, 3, 3, 300_000_000),
        Some(NaiveDate::from_ymd(2014, 4, 4).and_hms_nano(4, 4, 4, 400_000_000)),
        t_none,
    );
    _loghandle.parse_new_spec("info").unwrap();
    let result: Vec<MValue> = serde_db::ser::to_params(&input, &mut input_metadata.iter())?;

    let expected = vec![
        // int
        MValue::Short(0_i16),
        MValue::Short(1_i16),
        MValue::Short(2_i16),
        MValue::Short(3_i16),
        MValue::Null,
        // string
        MValue::String("Five".to_string()),
        MValue::String("Six".to_string()),
        MValue::String("Seven".to_string()),
        MValue::String("Eight".to_string()),
        MValue::Null,
        // timestamp
        MValue::Timestamp(mock_db::Timestamp(
            NaiveDate::from_ymd(2011, 1, 1).and_hms_nano(1, 1, 1, 100_000_000),
        )),
        MValue::Timestamp(mock_db::Timestamp(
            NaiveDate::from_ymd(2012, 2, 2).and_hms_nano(2, 2, 2, 200_000_000),
        )),
        MValue::Timestamp(mock_db::Timestamp(
            NaiveDate::from_ymd(2013, 3, 3).and_hms_nano(3, 3, 3, 300_000_000),
        )),
        MValue::Timestamp(mock_db::Timestamp(
            NaiveDate::from_ymd(2014, 4, 4).and_hms_nano(4, 4, 4, 400_000_000),
        )),
        MValue::Null,
    ];

    assert!(mvalvec_compare(&expected, &result));

    // panic!("Test Struct db values");
    Ok(())
}

fn struct_serialization(_loghandle: &mut LoggerHandle) -> mock_db::Result<()> {
    info!(
        "Test struct with all conversions (plain -> plain, Option -> plain, plain -> Option, \
         Option -> Option)"
    );
    let input_metadata: Vec<PT> = vec![
        // int
        PT::Short,
        PT::Short,
        PT::NullableShort,
        PT::NullableShort,
        PT::NullableShort,
        // string
        PT::String,
        PT::String,
        PT::NullableString,
        PT::NullableString,
        PT::NullableString,
        // timestamp
        PT::Timestamp,
        PT::Timestamp,
        PT::NullableTimestamp,
        PT::NullableTimestamp,
        PT::NullableTimestamp,
    ];
    let i_none: Option<i32> = None;
    let s_none: Option<String> = None;
    let t_none: Option<NaiveDateTime> = None;

    #[derive(Serialize)]
    struct Input {
        zero: i32,
        one: Option<i32>,
        two: i32,
        three: Option<i32>,
        four: Option<i32>,
        five: String,
        six: Option<String>,
        seven: String,
        eight: Option<String>,
        nine: Option<String>,
        eleven: NaiveDateTime,
        twelve: Option<NaiveDateTime>,
        thirteen: NaiveDateTime,
        fourteen: Option<NaiveDateTime>,
        fifteen: Option<NaiveDateTime>,
    }
    let input = Input {
        zero: 0_i32,
        one: Some(1_i32),
        two: 2_i32,
        three: Some(3_i32),
        four: i_none,
        five: "Five".to_string(),
        six: Some("Six".to_string()),
        seven: "Seven".to_string(),
        eight: Some("Eight".to_string()),
        nine: s_none,
        eleven: NaiveDate::from_ymd(2011, 1, 1).and_hms_nano(1, 1, 1, 100_000_000),
        twelve: Some(NaiveDate::from_ymd(2012, 2, 2).and_hms_nano(2, 2, 2, 200_000_000)),
        thirteen: NaiveDate::from_ymd(2013, 3, 3).and_hms_nano(3, 3, 3, 300_000_000),
        fourteen: Some(NaiveDate::from_ymd(2014, 4, 4).and_hms_nano(4, 4, 4, 400_000_000)),
        fifteen: t_none,
    };
    _loghandle.parse_new_spec("info").unwrap();
    let result: Vec<MValue> = serde_db::ser::to_params(&input, &mut input_metadata.iter())?;

    let expected = vec![
        MValue::Short(0_i16),
        MValue::Short(1_i16),
        MValue::Short(2_i16),
        MValue::Short(3_i16),
        MValue::Null,
        MValue::String("Five".to_string()),
        MValue::String("Six".to_string()),
        MValue::String("Seven".to_string()),
        MValue::String("Eight".to_string()),
        MValue::Null,
        MValue::Timestamp(mock_db::Timestamp(
            NaiveDate::from_ymd(2011, 1, 1).and_hms_nano(1, 1, 1, 100_000_000),
        )),
        MValue::Timestamp(mock_db::Timestamp(
            NaiveDate::from_ymd(2012, 2, 2).and_hms_nano(2, 2, 2, 200_000_000),
        )),
        MValue::Timestamp(mock_db::Timestamp(
            NaiveDate::from_ymd(2013, 3, 3).and_hms_nano(3, 3, 3, 300_000_000),
        )),
        MValue::Timestamp(mock_db::Timestamp(
            NaiveDate::from_ymd(2014, 4, 4).and_hms_nano(4, 4, 4, 400_000_000),
        )),
        MValue::Null,
    ];

    assert!(mvalvec_compare(&expected, &result));

    // panic!("Test Struct db values");
    Ok(())
}

fn mvalvec_compare(va: &[MValue], vb: &[MValue]) -> bool {
    (va.len() == vb.len()) &&  // zip stops at the shortest
     va.iter()
       .zip(vb)
       .all(|(a,b)| (*a==*b))
}
