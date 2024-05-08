#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::arch::global_asm;
use buddy_system_allocator::LockedHeap;
use log::{debug, LevelFilter};
use crate::driver::uart;
use crate::print::KernelLogger;

mod config;
mod symbols;
mod errors;
mod arch;
mod driver;
#[macro_use]
mod print;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

global_asm!(include_str!("asm/entry.S"));

static LOGGER: KernelLogger = KernelLogger;

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    uart::uart_init();
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Debug))
        .expect("Set logger error");
    debug_clear_bss();
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
