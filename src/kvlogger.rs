use crate::utils::*;
use colored::*;
use env_logger::filter::Filter;
use log::{Log, Metadata, Record};

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
