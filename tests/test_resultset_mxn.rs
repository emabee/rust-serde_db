extern crate chrono;
extern crate flexi_logger;
#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_db;
#[macro_use]
extern crate serde_derive;

mod mock_db;
mod util;

use chrono::NaiveDateTime;
#[allow(unused_imports)]
use flexi_logger::{LogSpecification, ReconfigurationHandle};
use mock_db::{MValue, Resultset, Timestamp};

const SIZE: usize = 20;

#[test] // cargo test --test test_resultset_mxn -- --nocapture
pub fn test_resultset_mxn() {
    let mut loghandle = util::init_logger("info");

    match evaluate_matrix_rs(&mut loghandle) {
        Err(e) => {
            error!("test_resultset_mxn() failed with {:?}", e);
            assert!(false)
        }
        Ok(_) => debug!("test_resultset_mxn() ended successful"),
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

fn evaluate_matrix_rs(loghandle: &mut ReconfigurationHandle) -> mock_db::Result<()> {
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
    pop_values_in_order(loghandle)?;
    not_rows_into_value(loghandle)?;

    Ok(())
}
// loghandle.set_new_spec(LogSpecification::parse("info"));

fn into_vec_struct(_loghandle: &mut ReconfigurationHandle) -> mock_db::Result<()> {
    info!("Convert a mxn resultset into a Vec<struct>");
    let vtd: Vec<TestData> = get_resultset_string_ts_short_short(SIZE).into_typed()?;
    assert_eq!(SIZE, vtd.len());
    for td in vtd {
        debug!("Got {}, {}, {}, {:?}", td.f1, td.f2, td.f3, td.f4);
    }
    Ok(())
}
fn into_vec_struct_options(_loghandle: &mut ReconfigurationHandle) -> mock_db::Result<()> {
    info!("Convert a mxn resultset into a Vec<struct>, check Option conversions");
    let vtd: Vec<TestOption> = get_resultset_option_option_short_short(SIZE).into_typed()?;
    assert_eq!(SIZE, vtd.len());
    for td in vtd {
        debug!("Got {:?}, {}, {:?}, {}", td.f1, td.f2, td.f3, td.f4);
    }
    Ok(())
}
fn not_into_vec_field(_loghandle: &mut ReconfigurationHandle) -> mock_db::Result<()> {
    let s = "Negative test: no conversion of nxm resultset into Vec<field>";
    info!("{}", s);
    let test: mock_db::Result<Vec<String>> = get_resultset_string_ts_short_short(SIZE).into_typed();
    match test {
        Ok(_) => assert!(false, "Failed \"{}\"", s),
        Err(e) => info!("--> Exception: {:?}", e),
    }
    Ok(())
}
fn not_into_struct(_loghandle: &mut ReconfigurationHandle) -> mock_db::Result<()> {
    let s = "Negative test: no conversion of nxm resultset into struct";
    info!("{}", s);
    let test: mock_db::Result<TestData> = get_resultset_string_ts_short_short(SIZE).into_typed();
    match test {
        Ok(_) => assert!(false, "Failed \"{}\"", s),
        Err(e) => info!("--> Exception: {:?}", e),
    }
    Ok(())
}
fn not_into_field(_loghandle: &mut ReconfigurationHandle) -> mock_db::Result<()> {
    let s = "Negative test: no conversion of nxm resultset into field";
    info!("{}", s);
    let test: mock_db::Result<String> = get_resultset_string_ts_short_short(SIZE).into_typed();
    match test {
        Ok(_) => assert!(false, "Failed \"{}\"", s),
        Err(e) => info!("--> Exception: {:?}", e),
    }
    Ok(())
}
fn rows_into_struct(_loghandle: &mut ReconfigurationHandle) -> mock_db::Result<()> {
    info!("Loop over rows (streaming support), convert row into struct");
    let mut sum: usize = 0;
    for row in get_resultset_string_ts_short_short(SIZE) {
        let td: TestData = row.into_typed()?;
        sum += td.f3 as usize;
    }
    assert_eq!(sum, SIZE * (SIZE + 1) / 2);
    Ok(())
}
fn not_into_short_struct(_loghandle: &mut ReconfigurationHandle) -> mock_db::Result<()> {
    let s = "Negative test: no conversion of mxn resultset into Vec<too short struct>";
    info!("{}", s);
    let test: mock_db::Result<Vec<ShortData>> =
        get_resultset_string_ts_short_short(SIZE).into_typed();
    match test {
        Ok(_) => assert!(false, "Failed \"{}\"", s),
        Err(e) => info!("--> Exception: {:?}", e),
    }
    Ok(())
}
fn rows_into_tuple(_loghandle: &mut ReconfigurationHandle) -> mock_db::Result<()> {
    let s = "Loop over rows, convert row into tuple";
    info!("{}", s);
    for row in get_resultset_string_ts_short_short(7) {
        let t: (String, NaiveDateTime, i32, Option<i32>) = row.into_typed()?;
        debug!("Got tuple with {}, {}, {}, {:?}", t.0, t.1, t.2, t.3);
    }
    Ok(())
}
fn not_rows_into_long_tuple(_loghandle: &mut ReconfigurationHandle) -> mock_db::Result<()> {
    let s = "Negative test: loop over rows, convert row into too long tuple";
    info!("{}", s);
    for row in get_resultset_string_ts_short_short(1) {
        let test: Result<(String, NaiveDateTime, i32, Option<i32>, i32), mock_db::Error> =
            row.into_typed();
        match test {
            Ok(_) => assert!(false, "Failed \"{}\"", s),
            Err(e) => info!("--> Exception: {:?}", e),
        }
    }
    Ok(())
}
fn rows_into_short_tuple(_loghandle: &mut ReconfigurationHandle) -> mock_db::Result<()> {
    let s = "Loop over rows, convert row into too short tuple";
    info!("{}", s);
    for row in get_resultset_string_ts_short_short(6) {
        let t: (String, NaiveDateTime, i32) = row.into_typed()?;
        debug!("Got tuple with {}, {}, {}", t.0, t.1, t.2);
    }
    Ok(())
}
fn rows_map_fold(_loghandle: &mut ReconfigurationHandle) -> mock_db::Result<()> {
    info!("Iterate over rows, map, fold");
    let sum = get_resultset_string_ts_short_short(SIZE)
        .into_iter()
        .map(|r| {
            let i: i32 = r.cloned_value(2).unwrap().into_typed().unwrap();
            i
        })
        .fold(0, |acc, i| acc + i);
    assert_eq!(sum as usize, SIZE * (SIZE + 1) / 2);
    Ok(())
}
fn pick_values_individually(_loghandle: &mut ReconfigurationHandle) -> mock_db::Result<()> {
    info!("Loop over rows, pick out single values individually, in arbitrary order");
    for row in get_resultset_string_ts_short_short(5) {
        let f2: NaiveDateTime = row.field_into_typed(1)?;
        let f1: String = row.field_into_typed(0)?;
        let f4: Option<i32> = Some(row.field_into_typed(3)?);
        let f3: i32 = row.field_into_typed(2)?;
        debug!("Got {}, {}, {}, {:?}", f1, f2, f3, f4);
    }
    Ok(())
}
fn pop_values_in_order(_loghandle: &mut ReconfigurationHandle) -> mock_db::Result<()> {
    info!("Loop over rows, pop single values, in reverse order");
    for mut row in get_resultset_string_ts_short_short(5) {
        let f4: Option<i32> = Some(row.pop_into_typed()?);
        let f3: i32 = row.pop_into_typed()?;
        let f2: NaiveDateTime = row.pop_into_typed()?;
        let f1: String = row.pop_into_typed()?;
        debug!("Got {}, {}, {}, {:?}", f1, f2, f3, f4);
    }
    Ok(())
}
fn not_rows_into_value(_loghandle: &mut ReconfigurationHandle) -> mock_db::Result<()> {
    let s = "Negative test: no conversion of row into field if two or more colums";
    info!("{}", s);
    for row in get_resultset_string_ts_short_short(1) {
        let test: Result<String, _> = row.into_typed();
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


////////////////////////////////////////////////////////
fn get_resultset_string_ts_short_short(len: usize) -> Resultset {
    assert!(len < 60);
    let mut rs = Resultset::new(&["f1", "f2", "f3", "f4"]);
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

fn get_resultset_option_option_short_short(len: usize) -> Resultset {
    let mut rs = Resultset::new(&["f1", "f2", "f3", "f4"]);
    for i in 0..len {
        rs.push(vec![
            MValue::NullableShort(if i % 2 == 0 { None } else { Some(i as i16) }),
            MValue::NullableShort(Some(i as i16)),
            MValue::Short(i as i16),
            MValue::Short(10 * i as i16 + 7),
        ]);
    }
    rs
}
