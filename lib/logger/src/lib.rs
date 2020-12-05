use flexi_logger::{colored_default_format, Logger};
pub use log;

pub fn init() {
  // Setup logger
  Logger::with_str("debug")
    .format(colored_default_format)
    .start_with_specfile("./logspec.toml")
    .unwrap();
}
