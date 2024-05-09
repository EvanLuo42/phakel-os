use lazy_static::lazy_static;
use crate::arch::riscv::qemu::layout::user::PAGE_SIZE;

use crate::memory::mapping::page_table_entry::PageTableEntry;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(4096))]
pub struct PageTable {
    pub entries: [PageTableEntry; PAGE_SIZE / 8]
}

lazy_static! {
    static ref KERNEL_PAGE_TABLE: PageTable = PageTable::empty();
}

impl PageTable {
    pub fn empty() -> Self {
        Self {
            entries: [PageTableEntry::new(0); PAGE_SIZE / 8]
        }
    }

    pub fn as_ptr(&self) -> usize {
        self.entries.as_ptr() as usize
    }

    #[inline]
    pub fn clear(&mut self) {
        self.entries
            .iter_mut()
            .for_each(PageTableEntry::write_zero);
    }

    #[inline]
    pub fn write(&mut self, pte: &PageTable) {
        self.entries
            .iter_mut()
            .enumerate()
            .for_each(|(i, x)| x.write(pte.entries[i].as_usize()))
    }
}