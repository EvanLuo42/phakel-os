pub const KERNEL_HEAP_SIZE: usize = 0x30_0000;

pub const PAGE_SIZE: usize = 0x1000;
pub const PAGE_SIZE_BITS: usize = 0xc;

pub const PHYS_ADDR_WIDTH: usize = 56;
pub const PHYS_PAGE_NUMBER_WIDTH: usize = PHYS_ADDR_WIDTH - PAGE_SIZE_BITS;

pub const VIRT_ADDR_WIDTH: usize = 39;
pub const VIRT_PAGE_NUMBER_WIDTH: usize = VIRT_ADDR_WIDTH - PAGE_SIZE_BITS;

pub const MEMORY_END: usize = 0x80800000;