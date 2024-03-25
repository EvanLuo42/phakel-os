use core::fmt::{Arguments, Write};

use log::{Level, Log, Metadata, Record};

use crate::sbi::console_write_byte;

struct StdOut;

impl Write for StdOut {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for char in s.chars() {
            console_write_byte(char as u8)
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> core::fmt::Result {
        console_write_byte(c as u8);
        Ok(())
    }
}

pub fn print(args: Arguments) {
    StdOut.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

pub struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} {}", get_color(record.level()), record.args());
        }
    }

    fn flush(&self) {
    }
}

fn get_color(level: Level) -> &'static str {
    match level {
        Level::Error => "\x1b[31mERROR\x1b[0m".into(),
        Level::Warn => "\x1b[93mWARN\x1b[0m".into(),
        Level::Info => "\x1b[34mINFO\x1b[0m".into(),
        Level::Debug => "\x1b[32mDEBUG\x1b[0m".into(),
        Level::Trace => "\x1b[90mTRACE\x1b[0m".into()
    }
}