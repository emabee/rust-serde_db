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

#[test] // cargo test --test test_resultset_1x1 -- --nocapture
pub fn test_resultset_1x1() {
    let mut loghandle = util::init_logger("info");

    match evaluate_field_rs(&mut loghandle) {
        Err(e) => {
            error!("test_resultset_1x1() failed with {:?}", e);
            assert!(false)
        }
        Ok(_) => debug!("test_resultset_1x1() ended successful"),
    }
}

// Test the various ways to evaluate a resultset
fn evaluate_field_rs(loghandle: &mut ReconfigurationHandle) -> MockResult<()> {
    #[derive(Deserialize)]
    struct TestDataMin {
        f1: String,
    };

    loghandle.set_new_spec(LogSpecification::parse("info"));
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
fn get_resultset_string(len: usize) -> MockResultset {
    assert!(len < 60);
    let mut rs = MockResultset::new(vec!["f1"]);
    for i in 0..len {
        rs.push(vec![MV::STRING(String::from_utf8(vec!['a' as u8 + i as u8]).unwrap())]);
    }
    rs
}
