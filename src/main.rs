#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::arch::global_asm;
use log::{debug, info, LevelFilter};
use crate::logger::KernelLogger;
use crate::mm::frame_allocator::{frame_allocator_test, init_frame_allocator};

mod lang_items;
mod sbi;
#[macro_use]
mod console;
mod logger;
mod config;
mod symbols;
mod mm;
mod errors;

global_asm!(include_str!("entry.asm"));

static LOGGER: KernelLogger = KernelLogger;

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Debug))
        .expect("Set logger error");
    debug_clear_bss();
    init_frame_allocator();
    frame_allocator_test();
    panic!("Shutdown machine!");
}

fn debug_clear_bss() {
    debug!(".text [{:#x}, {:#x})", symbols::stext as usize, symbols::etext as usize);
    debug!(".rodata [{:#x}, {:#x})", symbols::srodata as usize, symbols::erodata as usize);
    debug!(".data [{:#x}, {:#x})", symbols::sdata as usize, symbols::edata as usize);
}

fn clear_bss() {
    (symbols::sbss as usize..symbols::ebss as usize).for_each(|i| {
        unsafe { (i as *mut u8).write_volatile(0) }
    });
}
