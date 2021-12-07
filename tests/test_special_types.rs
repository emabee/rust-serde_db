#[macro_use]
extern crate log;

mod mock_db;
mod util;

use crate::mock_db::{MValue, Resultset};
use bigdecimal::BigDecimal;
#[allow(unused_imports)]
use flexi_logger::{LogSpecification, LoggerHandle};
use serde::Deserialize;

const SIZE: usize = 20;

#[test] // cargo test --test test_special_types -- --nocapture
pub fn test_special_types() {
    let mut loghandle = util::init_logger();

    match impl_test_special_types(&mut loghandle) {
        Err(e) => {
            error!("test_special_types() failed with {:?}", e);
            assert!(false)
        }
        Ok(_) => debug!("test_special_types() ended successful"),
    }
}

#[derive(Deserialize)]
struct TestData {
    f1: Option<BigDecimal>,
    f2: Option<BigDecimal>,
    f3: BigDecimal,
    f4: BigDecimal,
}

fn impl_test_special_types(loghandle: &mut LoggerHandle) -> mock_db::Result<()> {
    info!("=== Special Types ===");
    rs_single_fields(loghandle)?;
    rs_rows(loghandle)?;
    rs_resultset(loghandle)?;
    rs_single_value(loghandle)?;
    Ok(())
}

fn rs_single_fields(_loghandle: &mut LoggerHandle) -> mock_db::Result<()> {
    info!("Deserialization of single fields");
    let resultset = get_resultset_ooff(SIZE);
    assert_eq!(SIZE, resultset.len());
    for row in resultset {
        let f1: Option<BigDecimal> = row.field_into(0)?;
        let f2: Option<BigDecimal> = row.field_into(1)?;
        let f3: BigDecimal = row.field_into(2)?;
        let f4: BigDecimal = row.field_into(3)?;
        debug!("Got {:?}, {:?}, {:?}, {:?}", f1, f2, f3, f4);
    }
    Ok(())
}

fn rs_rows(_loghandle: &mut LoggerHandle) -> mock_db::Result<()> {
    info!("Deserialization of individual rows");
    let resultset = get_resultset_ooff(SIZE);
    assert_eq!(SIZE, resultset.len());
    for row in resultset {
        let td: TestData = row.try_into()?;
        debug!("Got {:?}, {:?}, {:?}, {:?}", td.f1, td.f2, td.f3, td.f4);
    }
    Ok(())
}

fn rs_resultset(_loghandle: &mut LoggerHandle) -> mock_db::Result<()> {
    info!("Deserialization of complete resultset");
    let vtd: Vec<TestData> = get_resultset_ooff(SIZE).try_into()?;
    assert_eq!(SIZE, vtd.len());
    for td in vtd {
        debug!("Got {:?}, {:?}, {:?}, {:?}", td.f1, td.f2, td.f3, td.f4);
    }
    Ok(())
}

fn rs_single_value(_loghandle: &mut LoggerHandle) -> mock_db::Result<()> {
    info!("Deserialization of complete resultset into a single special value");
    let value: Option<BigDecimal> = get_resultset_o1(false).try_into()?;
    assert_eq!(value, None);

    let value: Option<BigDecimal> = get_resultset_o1(true).try_into()?;
    assert_ne!(value, None);

    let _value: BigDecimal = get_resultset_o1(true).try_into()?;
    Ok(())
}

////////////////////////////////////////////////////////
fn get_resultset_ooff(len: usize) -> Resultset {
    let mut rs = Resultset::new(&["f1", "f2", "f3", "f4"]);
    for i in 0..len {
        rs.push(vec![
            if i % 2 == 0 {
                MValue::Null
            } else {
                MValue::Double(i as f64 * 0.01)
            },
            MValue::Double(i as f64 * 0.01),
            MValue::Double(i as f64),
            MValue::Double(10.0 * i as f64 + 3.456789),
        ]);
    }
    rs
}

fn get_resultset_o1(val: bool) -> Resultset {
    let mut rs = Resultset::new(&["f1"]);
    if val {
        rs.push(vec![MValue::Double(3.456_789_f64)]);
    } else {
        rs.push(vec![MValue::Null]);
    }
    rs
}
