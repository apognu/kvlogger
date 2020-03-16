use crate::kvlogger::KvLogger;
use env_logger::filter::{Builder as FilterBuilder, Filter};
use log::Level;
use std::error::Error;

pub struct KvLoggerBuilder {
  filter: Filter,
  datetime_format: String,
}

impl Default for KvLoggerBuilder {
  fn default() -> Self {
    Self {
      filter: FilterBuilder::from_env("RUST_LOG").build(),
      datetime_format: String::from("%Y-%m-%dT%H:%M:%S%z"),
    }
  }
}

impl KvLoggerBuilder {
  pub fn set_level(mut self, level: Level) -> KvLoggerBuilder {
    let spec = match level {
      Level::Trace => "trace",
      Level::Debug => "debug",
      Level::Info => "info",
      Level::Warn => "warn",
      Level::Error => "error",
    };

    self.filter = FilterBuilder::new().parse(spec).build();
    self
  }

  pub fn set_datetime_format<S>(mut self, format: S) -> KvLoggerBuilder
  where
    S: Into<String>,
  {
    self.datetime_format = format.into();
    self
  }

  pub fn init(self) -> Result<(), Box<dyn Error>> {
    let filter = self.filter.filter();
    let logger = KvLogger {
      filter: self.filter,
      datetime_format: self.datetime_format,
    };

    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(filter);

    Ok(())
  }
}
