use core::fmt::{Arguments, Write};

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