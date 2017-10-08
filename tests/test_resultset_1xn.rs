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

#[test] // cargo test --test test_resultset_1xn -- --nocapture
pub fn test_resultset_1xn() {
    let mut loghandle = util::init_logger("info");

    match evaluate_row_rs(&mut loghandle) {
        Err(e) => {
            error!("test_resultset_1xn() failed with {:?}", e);
            assert!(false)
        }
        Ok(_) => debug!("test_resultset_1xn() ended successful"),
    }
}

fn evaluate_row_rs(loghandle: &mut ReconfigurationHandle) -> MockResult<()> {
    #[derive(Deserialize)]
    struct TestData {
        f1: String,
        f4: Option<i32>,
        f3: i32,
        f2: NaiveDateTime,
    };

    loghandle.set_new_spec(LogSpecification::parse("info"));

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
