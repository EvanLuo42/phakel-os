use bitflags::bitflags;
use crate::memory::address::{Address, PhysicalAddress};

#[derive(Debug, Copy, Clone)]
pub struct PageTableEntry {
    bits: usize
}

bitflags! {
    #[derive(Debug, Copy, Clone)]
    pub struct PteFlags: usize {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
    }
}

impl PageTableEntry {
    #[inline]
    pub fn new(bits: usize) -> Self {
        Self {
            bits
        }
    }

    #[inline]
    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.as_usize() as *mut u8
    }

    #[inline]
    pub fn as_usize(&self) -> usize {
        self.bits
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        (self.bits & (PteFlags::V.bits())) > 0
    }

    #[inline]
    pub fn is_user(&self) -> bool {
        (self.bits & (PteFlags::U.bits())) > 0
    }

    #[inline]
    pub fn is_read(&self) -> bool {
        (self.bits & (PteFlags::R.bits())) > 0
    }

    #[inline]
    pub fn is_write(&self) -> bool {
        (self.bits & (PteFlags::W.bits())) > 0
    }

    #[inline]
    pub fn is_leaf(&self) -> bool {
        (self.bits & (PteFlags::R | PteFlags::W | PteFlags::X).bits()) != 0
    }

    #[inline]
    pub fn add_valid_bit(&mut self) {
        self.bits = self.as_usize() | (PteFlags::V.bits())
    }

    #[inline]
    pub fn add_user_bit(&mut self) {
        self.bits = self.as_usize() | (PteFlags::U.bits())
    }

    #[inline]
    pub fn rm_user_bit(&mut self) {
        self.bits &= !PteFlags::U.bits();
    }

    #[inline]
    pub fn as_flags(&self) -> usize {
        self.as_usize() & 0x3FF
    }

    #[inline]
    pub fn write_zero(&mut self) {
        self.bits = 0;
    }

    #[inline]
    pub fn write_perm(&mut self, pa: PhysicalAddress, perm: PteFlags) {
        self.bits = ((pa.as_usize() >> 12) << 10) | (perm | PteFlags::V).bits()
    }

    #[inline]
    pub fn write(&mut self, bits: usize) {
        self.bits = bits
    }
}

impl From<usize> for PageTableEntry {
    fn from(value: usize) -> Self {
        Self {
            bits: (value >> 12) << 10
        }
    }
}
