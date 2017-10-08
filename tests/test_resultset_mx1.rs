extern crate chrono;
extern crate flexi_logger;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_db;

mod util;

use flexi_logger::{ReconfigurationHandle, LogSpecification};
use util::{MockResult, MockResultset, MockValue as MV};

const SIZE: usize = 20;

#[test] // cargo test --test test_resultset_mx1 -- --nocapture
pub fn test_resultset_mx1() {
    let mut loghandle = util::init_logger("info");

    match evaluate_column_rs(&mut loghandle) {
        Err(e) => {
            error!("test_resultset_mx1() failed with {:?}", e);
            assert!(false)
        }
        Ok(_) => debug!("test_resultset_mx1() ended successful"),
    }
}

fn evaluate_column_rs(loghandle: &mut ReconfigurationHandle) -> MockResult<()> {
    #[derive(Deserialize)]
    struct TestDataMin {
        f1: String,
    };

    loghandle.set_new_spec(LogSpecification::parse("info"));

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

    {
        info!("Loop over rows, convert row with single field into struct");
        let mut acc = String::new();
        for row in get_resultset_string(7) {
            let td: TestDataMin = row.into_typed()?;
            if acc.len() != 0 {
                acc.push_str(", ")
            };
            acc.push_str(&td.f1);
        }
        assert_eq!(acc, "a, b, c, d, e, f, g");
    }

    {
        info!("Loop over rows, convert row into single value");
        for row in get_resultset_string(SIZE) {
            let f1: String = row.into_typed()?;
            debug!("Got single value: {}", f1);
        }
    }
    {
        info!("Iterate over rows, map, fold");
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
    }


    Ok(())
}

////////////////////////////////////////////////////////
fn get_resultset_string(len: usize) -> MockResultset {
    assert!(len < 60);
    let mut rs = MockResultset::new(vec!["f1"]);
    for i in 0..len {
        rs.push(vec![MV::STRING(String::from_utf8(vec!['a' as u8 + i as u8]).unwrap())]);
    }
    rs
}
