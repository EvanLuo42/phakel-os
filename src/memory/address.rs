use core::ops::Add;
use crate::arch::riscv::qemu::layout::user::{PAGE_MASK, PAGE_MASK_LEN, PAGE_SHIFT, PAGE_SIZE};

pub enum UnifiedAddress {
    UserPhysical,
    KernelPhysical,
    UserVirtual,
    KernelVirtual
}

pub trait Address {
    fn as_usize(&self) -> usize;

    fn as_data_ref(&self) -> &usize;

    fn as_data_mut(&mut self) -> &mut usize;

    #[inline]
    fn is_page_aligned(&self) -> bool {
        self.as_usize() % PAGE_SIZE == 0
    }

    #[inline]
    fn as_ptr(&self) -> *const u8 {
        self.as_usize() as *const u8
    }

    #[inline]
    fn as_mut_ptr(&self) -> *mut u8 {
        self.as_usize() as *mut u8
    }

    #[inline]
    fn page_round_up(&mut self) {
        *self.as_data_mut() = (*self.as_data_mut() + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)
    }

    #[inline]
    fn page_round_down(&mut self) {
        *self.as_data_mut() = *self.as_data_mut() & !(PAGE_SIZE - 1)
    }

    #[inline]
    fn add_page(&mut self) {
        *self.as_data_mut() += PAGE_SIZE;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Default)]
pub struct PhysicalAddress(pub usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Default)]
pub struct VirtualAddress(pub usize);

impl From<PhysicalAddress> for usize {
    fn from(pa: PhysicalAddress) -> Self {
        pa.0
    }
}

impl From<VirtualAddress> for usize {
    fn from(va: VirtualAddress) -> Self {
        va.0
    }
}

impl Address for VirtualAddress {
    #[inline]
    fn as_usize(&self) -> usize {
        self.0
    }

    #[inline]
    fn as_data_ref(&self) -> &usize {
        &self.0
    }

    #[inline]
    fn as_data_mut(&mut self) -> &mut usize {
        &mut self.0
    }
}

impl Address for PhysicalAddress {
    fn as_usize(&self) -> usize {
        self.0
    }

    #[inline]
    fn as_data_ref(&self) -> &usize {
        &self.0
    }

    #[inline]
    fn as_data_mut(&mut self) -> &mut usize {
        &mut self.0
    }
}

impl VirtualAddress {
    pub fn with_addr(addr: usize) -> Self {
        Self(addr)
    }

    pub fn page_num(&self, level: usize) -> usize {
        (self.0 >> (PAGE_SHIFT + level * PAGE_MASK_LEN)) & PAGE_MASK
    }
}

impl Add<usize> for VirtualAddress {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        VirtualAddress(self.0 + rhs)
    }
}

impl PhysicalAddress {
    pub fn with_addr(addr: usize) -> Self {
        Self(addr)
    }
}

impl Add<usize> for PhysicalAddress {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        PhysicalAddress(self.0 + rhs)
    }
}
