use flexi_logger::{Logger, ReconfigurationHandle};

pub fn init_logger(log_spec: &str) -> ReconfigurationHandle {
    Logger::with_str(log_spec).start_reconfigurable()
                              .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e))
}
