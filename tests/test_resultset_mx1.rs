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

use crate::mock_db::{MValue, Resultset};
#[allow(unused_imports)]
use flexi_logger::{LogSpecification, ReconfigurationHandle};

const SIZE: usize = 20;

#[test] // cargo test --test test_resultset_mx1 -- --nocapture
pub fn test_resultset_mx1() {
    let mut loghandle = util::init_logger();

    match evaluate_column_rs(&mut loghandle) {
        Err(e) => {
            error!("test_resultset_mx1() failed with {:?}", e);
            assert!(false)
        }
        Ok(_) => debug!("test_resultset_mx1() ended successful"),
    }
}

#[derive(Deserialize)]
struct TestDataMin {
    f1: String,
}

fn evaluate_column_rs(loghandle: &mut ReconfigurationHandle) -> mock_db::Result<()> {
    info!("=== Single column (mx1) ===");
    into_vec_struct(loghandle)?;
    into_vec_field(loghandle)?;
    not_into_struct(loghandle)?;
    not_into_field(loghandle)?;
    row_into_struct(loghandle)?;
    row_into_value(loghandle)?;
    row_map_fold(loghandle)?;
    Ok(())
}

// loghandle.set_new_spec(LogSpecification::parse("info"));

fn into_vec_struct(_loghandle: &mut ReconfigurationHandle) -> mock_db::Result<()> {
    info!("Convert a mx1 resultset into a Vec<struct>");
    let vec_d: Vec<TestDataMin> = get_resultset_string(SIZE).try_into()?;
    assert_eq!(SIZE, vec_d.len());
    for d in vec_d {
        debug!("Got {}", d.f1);
    }
    Ok(())
}

fn into_vec_field(_loghandle: &mut ReconfigurationHandle) -> mock_db::Result<()> {
    info!("Convert a mx1 resultset into a Vec<field>");
    let vec_s: Vec<String> = get_resultset_string(SIZE).try_into()?;
    assert_eq!(SIZE, vec_s.len());
    for s in vec_s {
        debug!("Got {}", s);
    }
    Ok(())
}
fn not_into_struct(_loghandle: &mut ReconfigurationHandle) -> mock_db::Result<()> {
    let s = "Negative test: no conversion of mx1 resultset into a struct";
    info!("{}", s);
    let test: mock_db::Result<TestDataMin> = get_resultset_string(SIZE).try_into();
    match test {
        Ok(_) => assert!(false, "Failed \"{}\"", s),
        Err(e) => info!("--> Exception: {:?}", e),
    }
    Ok(())
}
fn not_into_field(_loghandle: &mut ReconfigurationHandle) -> mock_db::Result<()> {
    let s = "Negative test: no conversion of mx1 resultset into a field";
    info!("{}", s);
    let test: mock_db::Result<String> = get_resultset_string(SIZE).try_into();
    match test {
        Ok(_) => assert!(false, "Failed \"{}\"", s),
        Err(e) => info!("--> Exception: {:?}", e),
    }
    Ok(())
}

fn row_into_struct(_loghandle: &mut ReconfigurationHandle) -> mock_db::Result<()> {
    info!("Loop over rows, convert row with single field into struct");
    let mut acc = String::new();
    for row in get_resultset_string(7) {
        let td: TestDataMin = row.try_into()?;
        if !acc.is_empty() {
            acc.push_str(", ")
        };
        acc.push_str(&td.f1);
    }
    assert_eq!(acc, "a, b, c, d, e, f, g");
    Ok(())
}

fn row_into_value(_loghandle: &mut ReconfigurationHandle) -> mock_db::Result<()> {
    info!("Loop over rows, convert row into single value");
    for row in get_resultset_string(SIZE) {
        let f1: String = row.try_into()?;
        debug!("Got single value: {}", f1);
    }
    Ok(())
}
fn row_map_fold(_loghandle: &mut ReconfigurationHandle) -> mock_db::Result<()> {
    info!("Iterate over rows, map, fold");
    let s = get_resultset_string(7)
        .into_iter()
        .map(|r| {
            let s: String = r.try_into().unwrap();
            s
        })
        .fold(String::new(), |mut acc, s| {
            if !acc.is_empty() {
                acc.push_str(", ")
            };
            acc.push_str(&s);
            acc
        });
    assert_eq!(s, "a, b, c, d, e, f, g");
    Ok(())
}

////////////////////////////////////////////////////////
fn get_resultset_string(len: usize) -> Resultset {
    assert!(len < 60);
    let mut rs = Resultset::new(&["f1"]);
    for i in 0..len {
        rs.push(vec![MValue::String(
            String::from_utf8(vec![b'a' + i as u8]).unwrap(),
        )]);
    }
    rs
}
