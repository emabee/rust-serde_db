extern crate chrono;
extern crate flexi_logger;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_db;

mod util;

use chrono::NaiveDateTime;
use util::{MockError, MockResult, MockResultset, MockValue as MV, MockTimestamp};

#[test] // cargo test --test test_resultset -- --nocapture
pub fn test_resultset() {
    util::init_logger("info");

    match impl_test_resultset() {
        Err(e) => {
            error!("impl_test_resultset() failed with {:?}", e);
            assert!(false)
        }
        Ok(_) => debug!("impl_test_resultset() ended successful"),
    }
}

// Test the various ways to evaluate a resultset
fn impl_test_resultset() -> MockResult<()> {
    evaluate_resultset()?;
    Ok(())
}

fn evaluate_resultset() -> MockResult<()> {
    #[derive(Debug, Deserialize)]
    struct TestData {
        f1: String,
        f2: NaiveDateTime,
        f3: i32,
        f4: Option<i32>,
    };

    #[derive(Debug, Deserialize)]
    struct TestDataMin {
        f1: String,
    };

    const SIZE: usize = 20;
    info!("Convert a mxn resultset into a Vec of structs");
    let vtd: Vec<TestData> = get_resultset_string_ts_short_short(SIZE).into_typed()?;
    assert_eq!(SIZE, vtd.len());
    for td in vtd {
        debug!("Got {}, {}, {}, {:?}", td.f1, td.f2, td.f3, td.f4);
    }

    info!("Convert a nx1 resultset into a Vec of fields");
    let vec_s: Vec<String> = get_resultset_string(SIZE).into_typed()?;
    assert_eq!(SIZE, vec_s.len());
    for s in vec_s {
        debug!("Got {}", s);
    }

    info!("Convert a 1x1 resultset into a single field");
    let s: String = get_resultset_string(1).into_typed()?;
    debug!("Got {}", s);


    info!("Loop over rows (streaming support), convert row into struct");
    let mut sum: usize = 0;
    for row in get_resultset_string_ts_short_short(SIZE) {
        let td: TestData = row.into_typed()?;
        sum += td.f3 as usize;
    }
    assert!(sum == SIZE * (SIZE + 1) / 2);


    let s = "Negative test: no conversion of nxm resultset into Vec<field>";
    info!("{}", s);
    let test: Result<Vec<String>, MockError> = get_resultset_string_ts_short_short(SIZE)
        .into_typed();
    if let Ok(_) = test {
        assert!(false, "Failed \"{}\"", s);
    }


    let s = "Negative test: no conversion of nxm resultset into field";
    info!("{}", s);
    let test: Result<String, MockError> = get_resultset_string_ts_short_short(SIZE).into_typed();
    if let Ok(_) = test {
        assert!(false, "Failed \"{}\"", s);
    }

    let s = "Negative test: no conversion of nx1 resultset into field";
    info!("{}", s);
    let test: Result<String, MockError> = get_resultset_string(SIZE).into_typed();
    if let Ok(_) = test {
        assert!(false, "Failed \"{}\" (1)", s);
    }

    let test: Result<i32, MockError> = get_resultset_string(SIZE).into_typed();
    if let Ok(_) = test {
        assert!(false, "Failed \"{}\" (2)", s);
    }

    Ok(())
}

////////////////////////////////////////////////////////
fn get_resultset_string_ts_short_short(len: usize) -> MockResultset {
    assert!(len < 60);
    let mut rs = MockResultset::new(vec!["f1", "f2", "f3", "f4"]);
    for i in 1..len + 1 {
        let s = format!("2017-09-{:02} 10:00:{:02}", i, i);
        let ts = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").unwrap();
        rs.push(vec![MV::STRING(String::from_utf8(vec!['a' as u8 + i as u8]).unwrap()),
                     MV::TIMESTAMP(MockTimestamp(ts)),
                     MV::SHORT(i as i16),
                     MV::SHORT(10 * i as i16 + 7)]);
    }
    trace!("Resultset = {:?}", rs);
    rs
}

fn get_resultset_string(len: usize) -> MockResultset {
    assert!(len < 60);
    let mut rs = MockResultset::new(vec!["f1"]);
    for i in 0..len {
        rs.push(vec![MV::STRING(String::from_utf8(vec!['a' as u8 + i as u8]).unwrap())]);
    }
    rs
}
