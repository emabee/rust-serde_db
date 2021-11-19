use flexi_logger::{Logger, LoggerHandle};

pub fn init_logger() -> LoggerHandle {
    Logger::with_str("info")
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e))
}
