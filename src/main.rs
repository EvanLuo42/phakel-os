#![no_std]
#![no_main]

use core::arch::global_asm;

mod lang_items;

global_asm!(include_str!("entry.asm"));
