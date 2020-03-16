use crate::kvlogger::KvLogger;
use env_logger::filter::{Builder as FilterBuilder, Filter};
use log::Level;
use std::error::Error;

/// A builder to create and register `kvlogger`
///
/// # Examples
///
/// ```
/// KvLogger::default().init?;
///
/// KvLogger::default()
///   .set_level(Level::Debug)
///   .set_datetime_format("%Y-%m-%d")
///   .init()?;
/// ```
pub struct KvLoggerBuilder {
  filter: Filter,
  datetime_format: String,
}

impl Default for KvLoggerBuilder {
  fn default() -> Self {
    Self {
      filter: FilterBuilder::from_env("RUST_LOG").build(),
      datetime_format: String::from("%Y-%m-%dT%H:%M:%S.%3f%z"),
    }
  }
}

impl KvLoggerBuilder {
  /// Force the maximum log level to be printed. If not given, the value of the
  /// `RUST_LOG` environment variable is considered.
  ///
  /// Defaults to `Error`.
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

  /// Change the datetime format used for timestamps
  ///
  /// This method does not have any effect unless the `datetime` feature is
  /// opted into. If it is not, the datetime timestamps is replaced with the
  /// current UNIX milliseconds timestamp.
  ///
  /// Defaults to `%Y-%m-%dT%H:%M:%S.3f%z`.
  pub fn set_datetime_format<S>(mut self, format: S) -> KvLoggerBuilder
  where
    S: Into<String>,
  {
    self.datetime_format = format.into();
    self
  }

  /// Finalize the builder and register the logger
  ///
  /// This method closes the builder by moving it and registers `kvlogger` as
  /// the default log system for the current program.
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
