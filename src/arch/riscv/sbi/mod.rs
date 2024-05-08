use sbi_rt::{NoReason, Shutdown, system_reset, SystemFailure, legacy};

pub fn console_write_byte(c: u8) {
    #[allow(deprecated)]
    legacy::console_putchar(c as usize);
}

pub fn shutdown(failure: bool) -> ! {
    if !failure {
        system_reset(Shutdown, NoReason);
    } else {
        system_reset(Shutdown, SystemFailure);
    }
    unreachable!()
}