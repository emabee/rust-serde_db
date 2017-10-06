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
use flexi_logger::ReconfigurationHandle;
use util::{MockResult, MockResultset, MockValue as MV, MockTimestamp};

#[test] // cargo test --test test_resultset -- --nocapture
pub fn test_row() {
    let loghandle = util::init_logger("info");

    match impl_test_resultset(&loghandle) {
        Err(e) => {
            error!("impl_test_resultset() failed with {:?}", e);
            assert!(false)
        }
        Ok(_) => debug!("impl_test_resultset() ended successful"),
    }
}

// Test the various ways to evaluate a resultset
fn impl_test_resultset(loghandle: &ReconfigurationHandle) -> MockResult<()> {
    evaluate_resultset(loghandle)?;
    Ok(())
}

fn evaluate_resultset(_loghandle: &ReconfigurationHandle) -> MockResult<()> {
    #[derive(Debug, Deserialize)]
    struct TestData {
        f1: String,
        f2: NaiveDateTime,
        f3: i32,
        f4: Option<i32>,
    };

    #[allow(dead_code)]
    struct ShortData {
        f1: String,
        f2: NaiveDateTime,
        f3: i32,
    };

    #[allow(dead_code)]
    struct LongData {
        f1: String,
        f2: NaiveDateTime,
        f3: i32,
        f4: Option<i32>,
        f5: Option<i32>,
    };

    #[derive(Debug, Deserialize)]
    struct TestDataMin {
        f1: String,
    };

    const SIZE: usize = 5;
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
            let i: i32 = r.cloned_value(2).unwrap().into_typed().unwrap();
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
        let f2: NaiveDateTime = row.field_into_typed(1)?;
        let f1: String = row.field_into_typed(0)?;
        let f4: Option<i32> = Some(row.field_into_typed(3)?);
        let f3: i32 = row.field_into_typed(2)?;
        debug!("Got {}, {}, {}, {:?}", f1, f2, f3, f4);
    }

    info!("Loop over rows, pick out single values individually, in reverse order");
    for mut row in get_resultset_string_ts_short_short(5) {
        let f4: Option<i32> = Some(row.pop_into_typed()?);
        let f3: i32 = row.pop_into_typed()?;
        let f2: NaiveDateTime = row.pop_into_typed()?;
        let f1: String = row.pop_into_typed()?;
        debug!("Got {}, {}, {}, {:?}", f1, f2, f3, f4);
    }

    info!("Negative test: no conversion of row into field if two or more colums");
    for row in get_resultset_string_ts_short_short(SIZE) {
        let test: Result<String, _> = row.into_typed();
        if let Ok(_) = test {
            assert!(false,
                    "Illegal conversion into a field for a row with two or more colums")
        }
    }

    // FIXME tests for Option<T> -> T and T -> Option<T>
    // FIXME tests for too big tuples or structs
    // FIXME tests for too small tuples or structs
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
