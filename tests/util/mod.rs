use flexi_logger::{Logger, LoggerHandle};

pub fn init_logger() -> LoggerHandle {
    Logger::try_with_str("info")
        .unwrap()
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e))
}
