use log::Level;

#[doc(hidden)]
pub fn get_decoration(level: Level) -> (&'static str, &'static str) {
  match level {
    Level::Trace => ("TRAC", ""),
    Level::Debug => ("DEBG", ""),
    Level::Info => ("INFO", "blue"),
    Level::Warn => ("WARN", "yellow"),
    Level::Error => ("FATA", "red"),
  }
}

#[doc(hidden)]
pub fn get_line(level: Level, key: &str, value: &str) -> String {
  use colored::*;

  let (_, color) = get_decoration(level);

  format!(r#"{}="{}""#, key.color(color).bold(), value)
}

#[cfg(feature = "datetime")]
#[doc(hidden)]
pub(crate) fn get_datetime(format: &str) -> String {
  format!("{}", chrono::Utc::now().format(format))
}

#[cfg(not(feature = "datetime"))]
#[doc(hidden)]
pub(crate) fn get_datetime(_: &str) -> String {
  use std::time::{SystemTime, UNIX_EPOCH};
  format!(
    "{}",
    SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .map(|t| t.as_millis())
      .unwrap_or(0)
  )
}
