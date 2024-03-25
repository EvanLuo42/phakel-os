#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;
use log::{debug, error, info, LevelFilter};
use crate::console::Logger;

mod lang_items;
mod sbi;
#[macro_use]
mod console;

global_asm!(include_str!("entry.asm"));

static LOGGER: Logger = Logger;

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Debug))
        .expect("Set logger error");
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
    }

    info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    debug!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    error!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}
