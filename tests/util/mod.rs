#![allow(dead_code)]

mod mock_error;
mod mock_resultset;
mod mock_row;
mod mock_value;

pub use self::mock_error::*;
pub use self::mock_value::*;
pub use self::mock_resultset::*;
pub use self::mock_row::*;

use flexi_logger::{Logger, ReconfigurationHandle};

pub fn init_logger(log_spec: &str) -> ReconfigurationHandle {
    Logger::with_str(log_spec)
        // .log_to_file()
        // .suppress_timestamp()
        .start_reconfigurable()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e))
}
