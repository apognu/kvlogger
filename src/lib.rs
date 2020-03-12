mod macros;

pub use self::macros::*;

use colored::*;
use env_logger::Builder;
use log::{Level, Log, Metadata, Record};
use std::error::Error;

struct KvLogger {
    level: Level,
}

impl Log for KvLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let (prefix, color) = get_decoration(record.metadata().level());
        let date = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%z");
        println!("{}[{}] {}", prefix.color(color).bold(), date, record.args());
    }

    fn flush(&self) {}
}

pub fn init() -> Result<(), Box<dyn Error>> {
    let logger = Builder::from_default_env().build();
    init_at(logger.filter().to_level().unwrap_or(Level::Info))?;

    Ok(())
}

pub fn init_at(level: Level) -> Result<(), Box<dyn Error>> {
    let logger = KvLogger { level };

    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(level.to_level_filter());

    Ok(())
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
