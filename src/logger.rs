use log::{Level, Log, Metadata, Record};
use crate::println;

pub struct KernelLogger;

impl Log for KernelLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("[kernel] {} - {}", get_color(record.level()), record.args());
        }
    }

    fn flush(&self) {
    }
}

fn get_color(level: Level) -> &'static str {
    match level {
        Level::Error => "\x1b[31mERROR\x1b[0m",
        Level::Warn => "\x1b[93mWARN\x1b[0m",
        Level::Info => "\x1b[34mINFO\x1b[0m",
        Level::Debug => "\x1b[32mDEBUG\x1b[0m",
        Level::Trace => "\x1b[90mTRACE\x1b[0m"
    }
}