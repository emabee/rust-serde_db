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

use crate::mock_db::{MValue, Resultset, Timestamp};
use chrono::NaiveDateTime;
use flexi_logger::ReconfigurationHandle;

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

fn evaluate_row_rs(loghandle: &mut ReconfigurationHandle) -> mock_db::Result<()> {
    #[derive(Deserialize)]
    struct TestData {
        f1: String,
        f4: Option<i32>,
        f3: i32,
        f2: NaiveDateTime,
    };

    loghandle.parse_new_spec("info");

    info!("=== Single row (1xn) ===");
    {
        info!("Convert a 1xn resultset into a Vec<struct>");
        let vtd: Vec<TestData> = get_resultset_string_ts_short_short(1).try_into()?;
        assert_eq!(1, vtd.len());
        for td in vtd {
            debug!("Got {}, {}, {}, {:?}", td.f1, td.f2, td.f3, td.f4);
        }
    }
    {
        info!("Convert a 1xn resultset into a struct");
        let td: TestData = get_resultset_string_ts_short_short(1).try_into()?;
        debug!("Got {}, {}, {}, {:?}", td.f1, td.f2, td.f3, td.f4);
    }
    {
        let s = "Negative test: no conversion of 1xn resultset into Vec<field>";
        info!("{}", s);
        let test: mock_db::Result<Vec<String>> =
            get_resultset_string_ts_short_short(SIZE).try_into();
        match test {
            Ok(_) => assert!(false, "Failed \"{}\"", s),
            Err(e) => info!("--> Exception: {:?}", e),
        }
    }
    {
        let s = "Negative test: no conversion of 1xn resultset into field";
        info!("{}", s);
        let test: mock_db::Result<String> = get_resultset_string_ts_short_short(SIZE).try_into();
        if test.is_ok() {
            assert!(false, "Failed \"{}\" (1)", s);
        }
        let test: mock_db::Result<i32> = get_resultset_string_ts_short_short(SIZE).try_into();
        match test {
            Ok(_) => assert!(false, "Failed \"{}\"", s),
            Err(e) => info!("--> Exception: {:?}", e),
        }
    }
    {
        info!("Convert a 1xn resultset into a Vec<(tuple)>");
        let vt: Vec<(String, NaiveDateTime, i32, Option<i32>)> =
            get_resultset_string_ts_short_short(1).try_into()?;
        assert_eq!(1, vt.len());
        for t in vt {
            debug!("Got {}, {}, {}, {:?}", t.0, t.1, t.2, t.3);
        }
    }
    {
        info!("Convert a 1xn resultset into a tuple");
        let t: (String, NaiveDateTime, i32, Option<i32>) =
            get_resultset_string_ts_short_short(1).try_into()?;
        debug!("Got {}, {}, {}, {:?}", t.0, t.1, t.2, t.3);
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
