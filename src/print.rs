use core::fmt;
use core::fmt::Write;
use core::panic::PanicInfo;
use log::{Level, Log, Metadata, Record};
use crate::arch::riscv::sbi::shutdown;
use crate::driver::uart::UART;

pub fn _print(args: fmt::Arguments) {
    UART.lock().write_fmt(args).unwrap()
}

pub fn console_put(c: u8) {
    UART.lock().put(c)
}

/// implement print and println! macro
///
/// use [`core::fmt::Write`] trait's [`console::Stdout`]
#[macro_export]
macro_rules! print {
    (fmt:literal$(, $($arg: tt)+)?) => {
        $crate::printf::console_putchar(format_args!($fmt(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt:literal$(, $($arg: tt)+)?) => {
        $crate::print::_print(format_args!(concat!($fmt, "\n") $(,$($arg)+)?));
    }
}

#[panic_handler]
fn panic(_panic_info: &PanicInfo) -> ! {
    if let Some(location) = _panic_info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            _panic_info.message().unwrap()
        );
    } else {
        println!("Panicked: {}", _panic_info.message().unwrap());
    }
    shutdown(true)
}

#[no_mangle]
fn abort() -> ! {
    panic!("abort");
}

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
