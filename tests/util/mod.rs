use flexi_logger::{Logger, ReconfigurationHandle};

pub fn init_logger() -> ReconfigurationHandle {
    Logger::with_str("info")
        .start_reconfigurable()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e))
}
