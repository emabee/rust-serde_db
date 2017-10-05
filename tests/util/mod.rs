#![allow(dead_code)]

mod mock_error;
mod mock_metadata;
mod mock_resultset;
mod mock_value;

pub use self::mock_error::*;
pub use self::mock_metadata::*;
pub use self::mock_value::*;
pub use self::mock_resultset::*;

use flexi_logger::Logger;

pub fn init_logger(log_spec: &str) {
    Logger::with_str(log_spec)
        // .log_to_file()
        // .suppress_timestamp()
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));
}
