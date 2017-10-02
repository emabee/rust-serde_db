extern crate chrono;
extern crate flexi_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde_db;

mod util;

use chrono::NaiveDateTime;
use util::{MockResult, MockResultset, MockValue as MV, MockTimestamp};

use serde_db::de::{DeserializableResultset, DeserializableRow, DeserError};

#[test] // cargo test --test test_resultset -- --nocapture
pub fn test_resultset() {
    util::init_logger("debug");

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

    const SIZE: usize = 5;
    info!("Convert a whole resultset into a Vec of structs");
    let vtd: Vec<TestData> = get_resultset_string_ts_short_short(7).into_typed()?;
    for td in vtd {
        debug!("Got {}, {}, {}, {:?}", td.f1, td.f2, td.f3, td.f4);
    }

    info!("Convert a whole resultset into a Vec of fields");
    let vec_s: Vec<String> = get_resultset_string(7).into_typed()?;
    for s in vec_s {
        debug!("Got {}", s);
    }

    info!("Convert a whole resultset into a single field");
    let s: String = get_resultset_string(1).into_typed()?;
    debug!("Got {}", s);


    info!("Loop over rows (streaming support), convert row into struct");
    let mut sum: usize = 0;
    for row in get_resultset_string_ts_short_short(SIZE) {
        let td: TestData = row.into_typed()?;
        sum += td.f3 as usize;
    }
    assert!(sum == SIZE * (SIZE + 1) / 2);


    info!("Negative test: no conversion of row into field if two or more colums");
    for row in get_resultset_string_ts_short_short(SIZE) {
        let test: Result<String, DeserError> = row.into_typed();
        if let Ok(_) = test {
            assert!(false,
                    "Illegal conversion into a field for a row with two or more colums")
        }
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

// fn get_resultset_string_string_short_short(len: usize) -> MockResultset {
//     assert!(len < 60);
//     let mut rs = MockResultset::new(vec!["f1", "f2", "f3", "f4"]);
//     for i in 1..len + 1 {
//         rs.push(vec![MV::STRING(String::from_utf8(vec!['a' as u8 + i as u8]).unwrap()),
//                      MV::STRING(String::from_utf8(vec!['A' as u8 + i as u8]).unwrap()),
//                      MV::SHORT(10 * i as i16 + 6),
//                      MV::SHORT(10 * i as i16 + 7)]);
//     }
//     trace!("Resultset = {:?}", rs);
//     rs
// }

fn get_resultset_string(len: usize) -> MockResultset {
    assert!(len < 60);
    let mut rs = MockResultset::new(vec!["f1"]);
    for i in 0..len {
        rs.push(vec![MV::STRING(String::from_utf8(vec!['a' as u8 + i as u8]).unwrap())]);
    }
    rs
}
