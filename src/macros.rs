#[macro_export]
macro_rules! kvlog {
  ( DONE, $level:expr, $message:expr, $kvs:expr ) => {
    let (prefix, color) = match $level {
      log::Level::Info => ("INFO", "blue"),
      log::Level::Warn => ("WARN", "yellow"),
      log::Level::Error => ("FATA", "red"),
      _ => ("", "")
    };

    let line = $kvs
      .iter()
      .map(|(k, v)| {
        format!(r#"{}="{}""#, k.color(color).bold(), v)
      })
      .collect::<Vec<String>>()
      .join(" ");

    let date = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%z");
    let line = format!("{}[{}] {} {}", prefix.color(color).bold(), date, $message, line);
    match $level {
      Level::Error => log::error!("{}", line),
      Level::Warn => log::warn!("{}", line),
      Level::Info => log::info!("{}", line),
      Level::Debug => log::debug!("{}", line),
      Level::Trace => log::trace!("{}", line),
    }
  };

  ( $level:ident, $message:expr, { $($key:expr => $value:expr),* } ) => {
    use colored::*;
    let mut kvs: std::collections::HashSet<(String, String)> = std::collections::HashSet::new();

    $(
      let k: String = $key.into();
      let v: String = $value.into();

      kvs.insert((k, v));
    )*

    kvlog!(DONE, log::Level::$level, $message, kvs);
  };
}
