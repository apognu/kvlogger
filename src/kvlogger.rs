use colored::*;
use env_logger::filter::Filter;
use log::{Level, Log, Metadata, Record};

pub(crate) struct KvLogger {
    pub(crate) filter: Filter,
    pub(crate) datetime_format: String,
}

impl Log for KvLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.filter.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let (prefix, color) = get_decoration(record.metadata().level());
        println!(
            "{}[{}] {}",
            prefix.color(color).bold(),
            get_datetime(&self.datetime_format),
            record.args()
        );
    }

    fn flush(&self) {}
}

pub fn get_decoration(level: Level) -> (&'static str, &'static str) {
    match level {
        Level::Trace => ("TRAC", ""),
        Level::Debug => ("DEBG", ""),
        Level::Info => ("INFO", "blue"),
        Level::Warn => ("WARN", "yellow"),
        Level::Error => ("FATA", "red"),
    }
}

pub fn get_line(level: Level, key: &str, value: &str) -> String {
    use colored::*;

    let (_, color) = get_decoration(level);

    format!(r#"{}="{}""#, key.color(color).bold(), value)
}

#[cfg(feature = "datetime")]
fn get_datetime(format: &str) -> String {
    format!("{}", chrono::Utc::now().format(format))
}

#[cfg(not(feature = "datetime"))]
fn get_datetime(_: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    format!(
        "{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|t| t.as_secs())
            .unwrap_or(0)
    )
}
