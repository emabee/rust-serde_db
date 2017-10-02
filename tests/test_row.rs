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

use serde_db::de::{DbValueInto, DeserializableRow, DeserError};

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

    const SIZE: usize = 5;
    // info!("Convert a whole resultset into a Vec of structs");
    // let vtd: Vec<TestData> = get_resultset_string_string_short_short(7).into_typed()?;
    // for td in vtd {
    //     debug!("Got {}, {}, {}, {:?}", td.f1, td.f2, td.f3, td.f4);
    // }

    info!("Loop over rows (streaming support), convert row into struct");
    let mut sum: usize = 0;
    for row in get_resultset_string_ts_short_short(SIZE) {
        let td: TestData = row.into_typed()?;
        sum += td.f3 as usize;
    }
    assert!(sum == SIZE * (SIZE + 1) / 2);

    info!("Loop over rows (streaming support), convert row with single field into struct");
    let mut acc = String::new();
    for row in get_resultset_string(7) {
        let td: TestDataMin = row.into_typed()?;
        if acc.len() != 0 {
            acc.push_str(", ")
        };
        acc.push_str(&td.f1);
    }
    assert_eq!(acc, "a, b, c, d, e, f, g");


    info!("Loop over rows (streaming support), convert row into tuple (avoid defining a struct)");
    for row in get_resultset_string_ts_short_short(7) {
        let t: (String, NaiveDateTime, i32, Option<i32>) = row.into_typed()?;
        debug!("Got tuple with {}, {}, {}, {:?}", t.0, t.1, t.2, t.3);
    }

    info!("Loop over rows (streaming support), convert row into single value");
    for row in get_resultset_string(SIZE) {
        let f1: String = row.into_typed()?;
        debug!("Got single value: {}", f1);
    }

    info!("Iterate over rows, map, fold");
    let sum = get_resultset_string_ts_short_short(SIZE)
        .into_iter()
        .map(|r| {
            let i: i32 = r.get(2).unwrap().clone().try_into().unwrap();
            i
        })
        .fold(0, |acc, i| acc + i);
    assert_eq!(sum as usize, SIZE * (SIZE + 1) / 2);

    let s = get_resultset_string(7)
        .into_iter()
        .map(|r| {
            let s: String = r.into_typed().unwrap();
            s
        })
        .fold(String::new(), |mut acc, s| {
            if acc.len() != 0 {
                acc.push_str(", ")
            };
            acc.push_str(&s);
            acc
        });
    assert_eq!(s, "a, b, c, d, e, f, g");

    info!("Loop over rows, pick out single values individually, in arbitrary order");
    for row in get_resultset_string_ts_short_short(5) {
        // FIXME this must be much easier to do!!
        let f2: NaiveDateTime = row.get(1).unwrap().clone().try_into()?;
        let f1: String = row.get(0).as_mut().unwrap().clone().try_into()?;
        let f4: Option<i32> = Some(row.get(3).unwrap().clone().try_into()?);
        let f3: i32 = row.get(2).unwrap().clone().try_into()?;
        debug!("Got {}, {}, {}, {:?}", f1, f2, f3, f4);
    }

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
