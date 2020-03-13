use colored::*;
use env_logger::{
    filter::{Builder as FilterBuilder, Filter},
    Builder,
};
use log::{Level, Log, Metadata, Record};
use std::error::Error;

struct KvLogger {
    filter: Filter,
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
        let date = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%z");
        println!("{}[{}] {}", prefix.color(color).bold(), date, record.args());
    }

    fn flush(&self) {}
}

pub fn init() -> Result<(), Box<dyn Error>> {
    let builder = Builder::from_default_env().build();
    let level = builder.filter().to_level().unwrap_or(Level::Error);

    let logger = KvLogger {
        filter: FilterBuilder::from_env("RUST_LOG").build(),
    };

    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(level.to_level_filter());

    Ok(())
}

pub fn init_at(level: Level) -> Result<(), Box<dyn Error>> {
    let spec = match level {
        Level::Trace => "trace",
        Level::Debug => "debug",
        Level::Info => "info",
        Level::Warn => "warn",
        Level::Error => "error",
    };

    let logger = KvLogger {
        filter: FilterBuilder::new().parse(spec).build(),
    };

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

pub fn get_line(level: Level, key: &str, value: &str) -> String {
    use colored::*;

    let (_, color) = get_decoration(level);

    format!(r#"{}="{}""#, key.color(color).bold(), value)
}
