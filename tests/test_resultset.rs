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
use util::{MockResult, MockResultset, MockValue as MV, MockTimestamp};

const SIZE: usize = 20;

#[test] // cargo test --test test_resultset -- --nocapture
pub fn test_resultset() {
    let mut loghandle = util::init_logger("info");

    match impl_test_resultset(&mut loghandle) {
        Err(e) => {
            error!("impl_test_resultset() failed with {:?}", e);
            assert!(false)
        }
        Ok(_) => debug!("impl_test_resultset() ended successful"),
    }
}

// Test the various ways to evaluate a resultset
fn impl_test_resultset(loghandle: &mut ReconfigurationHandle) -> MockResult<()> {
    evaluate_matrix_rs(loghandle)?;
    evaluate_column_rs(loghandle)?;
    evaluate_row_rs(loghandle)?;
    evaluate_field_rs(loghandle)?;
    Ok(())
}

fn evaluate_matrix_rs(loghandle: &mut ReconfigurationHandle) -> MockResult<()> {
    #[derive(Deserialize)]
    struct TestData {
        f1: String,
        f4: Option<i32>,
        f3: i32,
        f2: NaiveDateTime,
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
        let s = "Negative test: no conversion of mxn resultset into too short Vec<struct>";
        info!("{}", s);
        let test: MockResult<Vec<ShortData>> = get_resultset_string_ts_short_short(SIZE)
            .into_typed();
        match test {
            Ok(_) => assert!(false, "Failed \"{}\"", s),
            Err(e) => info!("--> Exception: {:?}", e),
        }
    }


    Ok(())
}

fn evaluate_column_rs(_loghandle: &mut ReconfigurationHandle) -> MockResult<()> {
    #[derive(Deserialize)]
    struct TestDataMin {
        f1: String,
    };
    info!("=== Single column (mx1) ===");
    {
        info!("Convert a mx1 resultset into a Vec<struct>");
        let vec_d: Vec<TestDataMin> = get_resultset_string(SIZE).into_typed()?;
        assert_eq!(SIZE, vec_d.len());
        for d in vec_d {
            debug!("Got {}", d.f1);
        }
    }
    {
        info!("Convert a mx1 resultset into a Vec<field>");
        let vec_s: Vec<String> = get_resultset_string(SIZE).into_typed()?;
        assert_eq!(SIZE, vec_s.len());
        for s in vec_s {
            debug!("Got {}", s);
        }
    }
    {
        let s = "Negative test: no conversion of mx1 resultset into a struct";
        info!("{}", s);
        let test: MockResult<TestDataMin> = get_resultset_string(SIZE).into_typed();
        match test {
            Ok(_) => assert!(false, "Failed \"{}\"", s),
            Err(e) => info!("--> Exception: {:?}", e),
        }
    }
    {
        let s = "Negative test: no conversion of mx1 resultset into a field";
        info!("{}", s);
        let test: MockResult<String> = get_resultset_string(SIZE).into_typed();
        match test {
            Ok(_) => assert!(false, "Failed \"{}\"", s),
            Err(e) => info!("--> Exception: {:?}", e),
        }
    }
    Ok(())
}

fn evaluate_row_rs(_loghandle: &mut ReconfigurationHandle) -> MockResult<()> {
    #[derive(Deserialize)]
    struct TestData {
        f1: String,
        f4: Option<i32>,
        f3: i32,
        f2: NaiveDateTime,
    };

    info!("=== Single row (1xn) ===");
    {
        info!("Convert a 1xn resultset into a Vec<struct>");
        let vtd: Vec<TestData> = get_resultset_string_ts_short_short(1).into_typed()?;
        assert_eq!(1, vtd.len());
        for td in vtd {
            debug!("Got {}, {}, {}, {:?}", td.f1, td.f2, td.f3, td.f4);
        }
    }
    {
        info!("Convert a 1xn resultset into a struct");
        let td: TestData = get_resultset_string_ts_short_short(1).into_typed()?;
        debug!("Got {}, {}, {}, {:?}", td.f1, td.f2, td.f3, td.f4);
    }
    {
        let s = "Negative test: no conversion of 1xn resultset into Vec<field>";
        info!("{}", s);
        let test: MockResult<Vec<String>> = get_resultset_string_ts_short_short(SIZE).into_typed();
        match test {
            Ok(_) => assert!(false, "Failed \"{}\"", s),
            Err(e) => info!("--> Exception: {:?}", e),
        }
    }
    {
        let s = "Negative test: no conversion of 1xn resultset into field";
        info!("{}", s);
        let test: MockResult<String> = get_resultset_string_ts_short_short(SIZE).into_typed();
        if let Ok(_) = test {
            assert!(false, "Failed \"{}\" (1)", s);
        }
        let test: MockResult<i32> = get_resultset_string_ts_short_short(SIZE).into_typed();
        match test {
            Ok(_) => assert!(false, "Failed \"{}\"", s),
            Err(e) => info!("--> Exception: {:?}", e),
        }
    }
    {
        info!("Convert a 1xn resultset into a Vec<(tuple)>");
        let vt: Vec<(String, NaiveDateTime, i32, Option<i32>)> =
            get_resultset_string_ts_short_short(1).into_typed()?;
        assert_eq!(1, vt.len());
        for t in vt {
            debug!("Got {}, {}, {}, {:?}", t.0, t.1, t.2, t.3);
        }
    }
    {
        info!("Convert a 1xn resultset into a tuple");
        let t: (String, NaiveDateTime, i32, Option<i32>) =
            get_resultset_string_ts_short_short(1).into_typed()?;
        debug!("Got {}, {}, {}, {:?}", t.0, t.1, t.2, t.3);
    }

    Ok(())
}

fn evaluate_field_rs(_loghandle: &mut ReconfigurationHandle) -> MockResult<()> {
    #[derive(Deserialize)]
    struct TestDataMin {
        f1: String,
    };
    info!("=== Single value (1x1) ===");
    {
        info!("Convert a 1x1 resultset into a Vec<struct>");
        let vd: Vec<TestDataMin> = get_resultset_string(1).into_typed()?;
        assert_eq!(vd.get(0).unwrap().f1, "a");
    }
    {
        info!("Convert a 1x1 resultset into a Vec<field>");
        let vs: Vec<String> = get_resultset_string(1).into_typed()?;
        assert_eq!(vs.get(0).unwrap(), "a");
    }
    {
        info!("Convert a 1x1 resultset into a struct");
        let d: TestDataMin = get_resultset_string(1).into_typed()?;
        assert_eq!(&d.f1, "a");
    }
    {
        info!("Convert a 1x1 resultset into a field");
        let s: String = get_resultset_string(1).into_typed()?;
        assert_eq!(&s, "a");
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

fn get_resultset_string(len: usize) -> MockResultset {
    assert!(len < 60);
    let mut rs = MockResultset::new(vec!["f1"]);
    for i in 0..len {
        rs.push(vec![MV::STRING(String::from_utf8(vec!['a' as u8 + i as u8]).unwrap())]);
    }
    rs
}
