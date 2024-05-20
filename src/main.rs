#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![allow(dead_code)]

extern crate alloc;

use core::arch::global_asm;
use buddy_system_allocator::LockedHeap;
use log::{debug, info, LevelFilter};
use crate::arch::riscv::RiscV;
use crate::print::KernelLogger;

mod config;
mod symbols;
mod errors;
mod arch;
#[macro_use]
mod print;

#[cfg(target_arch = "riscv64")]
global_asm!(include_str!("asm/riscv64_entry.S"));

#[global_allocator]
pub static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

static LOGGER: KernelLogger = KernelLogger;

#[cfg(target_arch = "riscv64")]
pub type GlobalArch = RiscV;

#[cfg(target_arch = "x86_64")]
pub type GlobalArch = RiscV;

mod uart {
    use lazy_static::lazy_static;
    use spin::mutex::SpinMutex;
    use crate::arch::{Arch, Uart};
    use crate::GlobalArch;
    lazy_static! {
        pub static ref UART: SpinMutex<<GlobalArch as Arch>::Uart> = SpinMutex::new(<GlobalArch as Arch>::Uart::new());
    }

    pub fn init() {
        UART.lock().init();
    }
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    uart::init();
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Debug))
        .expect("Set logger error");
    info!("Phakel OS v{}", env!("CARGO_PKG_VERSION"));
    println!("{}", include_str!("logo.txt"));
    info!("Initialized memory!");
    debug!(".text [{:#x}, {:#x})", symbols::stext as usize, symbols::etext as usize);
    debug!(".rodata [{:#x}, {:#x})", symbols::srodata as usize, symbols::erodata as usize);
    debug!(".data [{:#x}, {:#x})", symbols::sdata as usize, symbols::edata as usize);
    panic!("Shutdown machine!");
}
fn clear_bss() {
    (symbols::sbss as usize..symbols::ebss as usize).for_each(|i| {
        unsafe { (i as *mut u8).write_volatile(0) }
    });
}
