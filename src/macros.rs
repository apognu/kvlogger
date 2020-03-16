/// Log a message with a key/value list
///
/// Keys and values must implement the `Display` trait in order to be used. It
/// is recommended those keys and values output to a single line.
///
/// # Examples
///
/// ```
/// kvlog!(Info, "a user just logged in", {
///   "username" => "apognu",
///   "status" => 200
/// })
/// ```
#[macro_export]
macro_rules! kvlog {
  ( DONE, $level:expr, $message:expr, $kvs:expr ) => {
    let line = $kvs
      .iter()
      .map(|(k, v)| kvlogger::get_line($level, k, v))
      .collect::<Vec<String>>()
      .join(" ");

    let line = format!("{} {}", $message, line);
    match $level {
      log::Level::Error => log::error!("{}", line),
      log::Level::Warn => log::warn!("{}", line),
      log::Level::Info => log::info!("{}", line),
      log::Level::Debug => log::debug!("{}", line),
      log::Level::Trace => log::trace!("{}", line),
    }
  };

  ( $level:ident, $message:expr, { $($key:expr => $value:expr),* } ) => {
    let mut kvs: std::collections::HashSet<(String, String)> = std::collections::HashSet::new();

    $(
      let k: String = format!("{}", $key);
      let v: String = format!("{}", $value);

      kvs.insert((k, v));
    )*

    kvlog!(DONE, log::Level::$level, $message, kvs);
  };

  ( $level:ident, $message:expr ) => {
    kvlog!($level, $message, {});
  };
}
