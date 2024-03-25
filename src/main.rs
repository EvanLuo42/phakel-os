#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::arch::global_asm;
use log::{debug, info, LevelFilter};
use crate::logger::KernelLogger;
use crate::mm::heap_allocator::init_heap;

mod lang_items;
mod sbi;
#[macro_use]
mod console;
mod logger;
mod config;
mod mm;

global_asm!(include_str!("entry.asm"));

static LOGGER: KernelLogger = KernelLogger;

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Debug))
        .expect("Set logger error");
    debug_clear_bss();
    info!("Initializing Heap...");
    init_heap();
    info!("Heap Initialization Finished!");
    panic!("Shutdown machine!");
}

fn debug_clear_bss() {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
    }

    debug!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    debug!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    debug!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    (sbss as usize..ebss as usize).for_each(|i| {
        unsafe { (i as *mut u8).write_volatile(0) }
    });
}
