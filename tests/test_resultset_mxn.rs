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
use flexi_logger::{ReconfigurationHandle, LogSpecification};
use util::{MockError, MockResult, MockResultset, MockValue as MV, MockTimestamp};

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

fn evaluate_matrix_rs(loghandle: &mut ReconfigurationHandle) -> MockResult<()> {
    #[derive(Deserialize)]
    struct TestData {
        f1: String,
        f4: Option<i32>,
        f3: i32,
        f2: NaiveDateTime,
    };
    #[derive(Deserialize)]
    struct TestOption {
        f1: Option<i32>,
        f2: i32,
        f3: Option<i32>,
        f4: i32,
    };
    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct ShortData {
        f1: String,
        f2: NaiveDateTime,
        f4: Option<i32>,
    };
    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct LongData {
        f1: String,
        f2: NaiveDateTime,
        f3: i32,
        f4: Option<i32>,
        f5: Option<i32>,
    };

    loghandle.set_new_spec(LogSpecification::parse("info"));

    info!("=== Matrix (mxn) ===");
    {
        info!("Convert a mxn resultset into a Vec<struct>");
        let vtd: Vec<TestData> = get_resultset_string_ts_short_short(SIZE).into_typed()?;
        assert_eq!(SIZE, vtd.len());
        for td in vtd {
            debug!("Got {}, {}, {}, {:?}", td.f1, td.f2, td.f3, td.f4);
        }
    }
    {
        info!("Convert a mxn resultset into a Vec<struct>, check Option conversions");
        let vtd: Vec<TestOption> = get_resultset_option_option_short_short(SIZE).into_typed()?;
        assert_eq!(SIZE, vtd.len());
        for td in vtd {
            debug!("Got {:?}, {}, {:?}, {}", td.f1, td.f2, td.f3, td.f4);
        }
    }
    {
        let s = "Negative test: no conversion of nxm resultset into Vec<field>";
        info!("{}", s);
        let test: MockResult<Vec<String>> = get_resultset_string_ts_short_short(SIZE).into_typed();
        match test {
            Ok(_) => assert!(false, "Failed \"{}\"", s),
            Err(e) => info!("--> Exception: {:?}", e),
        }
    }
    {
        let s = "Negative test: no conversion of nxm resultset into struct";
        info!("{}", s);
        let test: MockResult<TestData> = get_resultset_string_ts_short_short(SIZE).into_typed();
        match test {
            Ok(_) => assert!(false, "Failed \"{}\"", s),
            Err(e) => info!("--> Exception: {:?}", e),
        }
    }
    {
        let s = "Negative test: no conversion of nxm resultset into field";
        info!("{}", s);
        let test: MockResult<String> = get_resultset_string_ts_short_short(SIZE).into_typed();
        match test {
            Ok(_) => assert!(false, "Failed \"{}\"", s),
            Err(e) => info!("--> Exception: {:?}", e),
        }
    }
    {
        info!("Loop over rows (streaming support), convert row into struct");
        let mut sum: usize = 0;
        for row in get_resultset_string_ts_short_short(SIZE) {
            let td: TestData = row.into_typed()?;
            sum += td.f3 as usize;
        }
        assert!(sum == SIZE * (SIZE + 1) / 2);
    }

    {
        let s = "Negative test: no conversion of mxn resultset into Vec<too short struct>";
        info!("{}", s);
        let test: MockResult<Vec<ShortData>> = get_resultset_string_ts_short_short(SIZE)
            .into_typed();
        match test {
            Ok(_) => assert!(false, "Failed \"{}\"", s),
            Err(e) => info!("--> Exception: {:?}", e),
        }
    }

    {
        let s = "Loop over rows, convert row into tuple";
        info!("{}", s);
        for row in get_resultset_string_ts_short_short(7) {
            let t: (String, NaiveDateTime, i32, Option<i32>) = row.into_typed()?;
            debug!("Got tuple with {}, {}, {}, {:?}", t.0, t.1, t.2, t.3);
        }
    }
    {
        let s = "Negative test: loop over rows, convert row into too long tuple";
        info!("{}", s);
        for row in get_resultset_string_ts_short_short(1) {
            let test: Result<(String, NaiveDateTime, i32, Option<i32>, i32), MockError> =
                row.into_typed();
            match test {
                Ok(_) => assert!(false, "Failed \"{}\"", s),
                Err(e) => info!("--> Exception: {:?}", e),
            }
        }
    }
    {
        let s = "Loop over rows, convert row into too short tuple";
        info!("{}", s);
        for row in get_resultset_string_ts_short_short(6) {
            let t: (String, NaiveDateTime, i32) = row.into_typed()?;
            debug!("Got tuple with {}, {}, {}", t.0, t.1, t.2);
        }
    }
    {
        info!("Iterate over rows, map, fold");
        let sum = get_resultset_string_ts_short_short(SIZE)
            .into_iter()
            .map(|r| {
                let i: i32 = r.cloned_value(2).unwrap().into_typed().unwrap();
                i
            })
            .fold(0, |acc, i| acc + i);
        assert_eq!(sum as usize, SIZE * (SIZE + 1) / 2);
    }
    {
        info!("Loop over rows, pick out single values individually, in arbitrary order");
        for row in get_resultset_string_ts_short_short(5) {
            let f2: NaiveDateTime = row.field_into_typed(1)?;
            let f1: String = row.field_into_typed(0)?;
            let f4: Option<i32> = Some(row.field_into_typed(3)?);
            let f3: i32 = row.field_into_typed(2)?;
            debug!("Got {}, {}, {}, {:?}", f1, f2, f3, f4);
        }
    }
    {
        info!("Loop over rows, pick out single values individually, in reverse order");
        for mut row in get_resultset_string_ts_short_short(5) {
            let f4: Option<i32> = Some(row.pop_into_typed()?);
            let f3: i32 = row.pop_into_typed()?;
            let f2: NaiveDateTime = row.pop_into_typed()?;
            let f1: String = row.pop_into_typed()?;
            debug!("Got {}, {}, {}, {:?}", f1, f2, f3, f4);
        }
    }
    {
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
    rs
}

fn get_resultset_option_option_short_short(len: usize) -> MockResultset {
    let mut rs = MockResultset::new(vec!["f1", "f2", "f3", "f4"]);
    for i in 0..len {
        rs.push(vec![MV::NULLABLESHORT(if i % 2 == 0 { None } else { Some(i as i16) }),
                     MV::NULLABLESHORT(Some(i as i16)),
                     MV::SHORT(i as i16),
                     MV::SHORT(10 * i as i16 + 7)]);
    }
    rs
}
