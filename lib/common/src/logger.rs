use flexi_logger::Logger;
pub use log;

pub fn init_logger() {
  // Setup logger
  Logger::try_with_str("info").unwrap().start().unwrap();
}
