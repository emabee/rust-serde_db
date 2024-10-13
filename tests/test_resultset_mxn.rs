#[macro_use]
extern crate log;

mod mock_db;
mod util;

use crate::mock_db::{MValue, ResultSet, Timestamp};
use chrono::NaiveDateTime;
#[allow(unused_imports)]
use flexi_logger::{LogSpecification, LoggerHandle};
use serde::Deserialize;

const SIZE: usize = 20;

#[test] // cargo test --test test_result_set_mxn -- --nocapture
pub fn test_result_set_mxn() {
    let mut loghandle = util::init_logger();

    match evaluate_matrix_rs(&mut loghandle) {
        Err(e) => {
            error!("test_result_set_mxn() failed with {:?}", e);
            assert!(false)
        }
        Ok(_) => debug!("test_result_set_mxn() ended successful"),
    }
}

#[derive(Deserialize)]
struct TestData {
    f1: String,
    f4: Option<i32>,
    f3: i32,
    f2: NaiveDateTime,
}
#[derive(Deserialize)]
struct TestOption {
    f1: Option<i32>,
    f2: i32,
    f3: Option<i32>,
    f4: i32,
}
#[allow(dead_code)]
#[derive(Deserialize)]
struct ShortData {
    f1: String,
    f2: NaiveDateTime,
    f4: Option<i32>,
}
#[allow(dead_code)]
#[derive(Deserialize)]
struct LongData {
    f1: String,
    f2: NaiveDateTime,
    f3: i32,
    f4: Option<i32>,
    f5: Option<i32>,
}

fn evaluate_matrix_rs(loghandle: &mut LoggerHandle) -> mock_db::Result<()> {
    info!("=== Matrix (mxn) ===");
    into_vec_struct(loghandle)?;
    into_vec_struct_options(loghandle)?;
    not_into_vec_field(loghandle)?;
    not_into_struct(loghandle)?;
    not_into_field(loghandle)?;
    rows_into_struct(loghandle)?;
    not_into_short_struct(loghandle)?;
    rows_into_tuple(loghandle)?;
    not_rows_into_long_tuple(loghandle)?;
    rows_into_short_tuple(loghandle)?;
    rows_map_fold(loghandle)?;
    pick_values_individually(loghandle)?;
    convert_values_one_by_one(loghandle)?;
    not_row_into_value(loghandle)?;
    not_rows_into_tuple(loghandle)?;
    not_rows_into_vec_of_short_struct(loghandle)?;
    not_rows_into_vec_of_long_tuple(loghandle)?;

    Ok(())
}
// loghandle.set_new_spec(LogSpecification::parse("info"));

fn into_vec_struct(_loghandle: &mut LoggerHandle) -> mock_db::Result<()> {
    info!("Convert a mxn result set into a Vec<struct>");
    let vtd: Vec<TestData> = get_result_set_string_ts_short_short(SIZE).try_into()?;
    assert_eq!(SIZE, vtd.len());
    for td in vtd {
        debug!("Got {}, {}, {}, {:?}", td.f1, td.f2, td.f3, td.f4);
    }
    Ok(())
}
fn into_vec_struct_options(_loghandle: &mut LoggerHandle) -> mock_db::Result<()> {
    info!("Convert a mxn result set into a Vec<struct>, check Option conversions");
    let vtd: Vec<TestOption> = get_result_set_option_option_short_short(SIZE).try_into()?;
    assert_eq!(SIZE, vtd.len());
    for td in vtd {
        debug!("Got {:?}, {}, {:?}, {}", td.f1, td.f2, td.f3, td.f4);
    }
    Ok(())
}
fn not_into_vec_field(_loghandle: &mut LoggerHandle) -> mock_db::Result<()> {
    let s = "Negative test: no conversion of nxm result set into Vec<field>";
    info!("{}", s);
    let test: mock_db::Result<Vec<String>> = get_result_set_string_ts_short_short(SIZE).try_into();
    match test {
        Ok(_) => assert!(false, "Failed \"{}\"", s),
        Err(e) => info!("--> Exception: {:?}", e),
    }
    Ok(())
}
fn not_into_struct(_loghandle: &mut LoggerHandle) -> mock_db::Result<()> {
    let s = "Negative test: no conversion of nxm result set into struct";
    info!("{}", s);
    let test: mock_db::Result<TestData> = get_result_set_string_ts_short_short(SIZE).try_into();
    match test {
        Ok(_) => assert!(false, "Failed \"{}\"", s),
        Err(e) => info!("--> Exception: {:?}", e),
    }
    Ok(())
}
fn not_into_field(_loghandle: &mut LoggerHandle) -> mock_db::Result<()> {
    let s = "Negative test: no conversion of nxm result set into field";
    info!("{}", s);
    let test: mock_db::Result<String> = get_result_set_string_ts_short_short(SIZE).try_into();
    match test {
        Ok(_) => assert!(false, "Failed \"{}\"", s),
        Err(e) => info!("--> Exception: {:?}", e),
    }
    Ok(())
}
fn rows_into_struct(_loghandle: &mut LoggerHandle) -> mock_db::Result<()> {
    info!("Loop over rows (streaming support), convert row into struct");
    let mut sum: usize = 0;
    for row in get_result_set_string_ts_short_short(SIZE) {
        let td: TestData = row.try_into()?;
        sum += td.f3 as usize;
    }
    assert_eq!(sum, SIZE * (SIZE + 1) / 2);
    Ok(())
}
fn not_into_short_struct(_loghandle: &mut LoggerHandle) -> mock_db::Result<()> {
    let s = "Negative test: no conversion of mxn result set into Vec<too short struct>";
    info!("{}", s);
    let test: mock_db::Result<Vec<ShortData>> =
        get_result_set_string_ts_short_short(SIZE).try_into();
    match test {
        Ok(_) => assert!(false, "Failed \"{}\"", s),
        Err(e) => info!("--> Exception: {:?}", e),
    }
    Ok(())
}
fn rows_into_tuple(_loghandle: &mut LoggerHandle) -> mock_db::Result<()> {
    let s = "Loop over rows, convert row into tuple";
    info!("{}", s);
    for row in get_result_set_string_ts_short_short(7) {
        let t: (String, NaiveDateTime, i32, Option<i32>) = row.try_into()?;
        debug!("Got tuple with {}, {}, {}, {:?}", t.0, t.1, t.2, t.3);
    }
    Ok(())
}
fn not_rows_into_long_tuple(_loghandle: &mut LoggerHandle) -> mock_db::Result<()> {
    let s = "Negative test: loop over rows, convert row into too long tuple";
    info!("{}", s);
    for row in get_result_set_string_ts_short_short(1) {
        let test: Result<(String, NaiveDateTime, i32, Option<i32>, i32), mock_db::Error> =
            row.try_into();
        match test {
            Ok(_) => assert!(false, "Failed \"{}\"", s),
            Err(e) => info!("--> Exception: {:?}", e),
        }
    }
    Ok(())
}
fn rows_into_short_tuple(_loghandle: &mut LoggerHandle) -> mock_db::Result<()> {
    let s = "Loop over rows, convert row into too short tuple";
    info!("{}", s);
    for row in get_result_set_string_ts_short_short(6) {
        let t: (String, NaiveDateTime, i32) = row.try_into()?;
        debug!("Got tuple with {}, {}, {}", t.0, t.1, t.2);
    }
    Ok(())
}
fn rows_map_fold(_loghandle: &mut LoggerHandle) -> mock_db::Result<()> {
    info!("Iterate over rows, map, fold");
    let sum = get_result_set_string_ts_short_short(SIZE)
        .into_iter()
        .map(|r| {
            let i: i32 = r.cloned_value(2).unwrap().try_into().unwrap();
            i
        })
        .fold(0, |acc, i| acc + i);
    assert_eq!(sum as usize, SIZE * (SIZE + 1) / 2);
    Ok(())
}
fn pick_values_individually(_loghandle: &mut LoggerHandle) -> mock_db::Result<()> {
    info!("Loop over rows, pick out single values individually, in arbitrary order");
    for row in get_result_set_string_ts_short_short(5) {
        let f2: NaiveDateTime = row.field_into(1)?;
        let f1: String = row.field_into(0)?;
        let f4: Option<i32> = row.field_into(3)?;
        let f3: i32 = row.field_into(2)?;
        debug!("Got {}, {}, {}, {:?}", f1, f2, f3, f4);
    }
    Ok(())
}

fn convert_values_one_by_one(_loghandle: &mut LoggerHandle) -> mock_db::Result<()> {
    info!("Loop over rows, convert single values");
    for mut row in get_result_set_string_ts_short_short(5) {
        let f1: String = row.next_try_into()?;
        let f2: NaiveDateTime = row.next_try_into()?;
        let f3: i32 = row.next_try_into()?;
        let f4: Option<i32> = Some(row.next_try_into()?);
        debug!("Got {}, {}, {}, {:?}", f1, f2, f3, f4);
    }
    Ok(())
}

fn not_rows_into_tuple(_loghandle: &mut LoggerHandle) -> mock_db::Result<()> {
    let s = "Negative test: no conversion of multiple rows into tuple";
    info!("{}", s);
    let result_set = get_result_set_string_ts_short_short(4);
    let test: Result<(String, chrono::NaiveDateTime, i32, i32), _> = result_set.try_into();
    match test {
        Ok(_) => assert!(false, "Deserialization was unexpectedly successful"),
        Err(e) => {
            info!("--> Exception: {:?}", e);
        }
    }
    Ok(())
}

fn not_row_into_value(_loghandle: &mut LoggerHandle) -> mock_db::Result<()> {
    let s = "Negative test: no conversion of row into field if two or more colums";
    info!("{}", s);
    for row in get_result_set_string_ts_short_short(1) {
        let test: Result<String, _> = row.try_into();
        match test {
            Ok(_) => assert!(false, "Failed \"{}\"", s),
            Err(e) => {
                info!("--> Exception: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}

fn not_rows_into_vec_of_short_struct(_loghandle: &mut LoggerHandle) -> mock_db::Result<()> {
    let s = "Negative test: no conversion of rows into vec of too short struct";
    info!("{}", s);
    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    struct TestData {
        f1: String,
        f2: NaiveDateTime,
    }
    let result_set = get_result_set_string_ts_short_short(4);
    let test: Result<Vec<TestData>, _> = result_set.try_into();
    match test {
        Ok(test) => assert!(false, "Could deserialize \"{:?}\"", test),
        Err(e) => {
            info!("--> Exception: {:?}", e);
        }
    }

    Ok(())
}

fn not_rows_into_vec_of_long_tuple(_loghandle: &mut LoggerHandle) -> mock_db::Result<()> {
    let s = "Negative test: no conversion of rows into vec of too long tuple";
    info!("{}", s);
    let result_set = get_result_set_string_ts_short_short(4);
    let test: Result<Vec<(String, NaiveDateTime, i32, i32, i32)>, _> = result_set.try_into();
    match test {
        Ok(_) => assert!(false, "Failed \"{:?}\"", s),
        Err(e) => {
            info!("--> Exception: {:?}", e);
        }
    }

    Ok(())
}

////////////////////////////////////////////////////////
fn get_result_set_string_ts_short_short(len: usize) -> ResultSet {
    assert!(len < 60);
    let mut rs = ResultSet::new(&["f1", "f2", "f3", "f4"]);
    for i in 1..len + 1 {
        let s = format!("2017-09-{:02} 10:00:{:02}", i, i);
        let ts = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").unwrap();
        rs.push(vec![
            MValue::String(String::from_utf8(vec![b'a' + i as u8]).unwrap()),
            MValue::Timestamp(Timestamp(ts)),
            MValue::Short(i as i16),
            MValue::Short(10 * i as i16 + 7),
        ]);
    }
    rs
}

fn get_result_set_option_option_short_short(len: usize) -> ResultSet {
    let mut rs = ResultSet::new(&["f1", "f2", "f3", "f4"]);
    for i in 0..len {
        rs.push(vec![
            if i % 2 == 0 {
                MValue::Null
            } else {
                MValue::Short(i as i16)
            },
            MValue::Short(i as i16),
            MValue::Short(i as i16),
            MValue::Short(10 * i as i16 + 7),
        ]);
    }
    rs
}
