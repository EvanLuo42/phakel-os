use alloc::string::String;
use crate::config::{PAGE_SIZE, PAGE_SIZE_BITS, PHYS_ADDR_WIDTH, PHYS_PAGE_NUMBER_WIDTH, VIRT_ADDR_WIDTH, VIRT_PAGE_NUMBER_WIDTH};
use crate::errors::KernelError;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct PhysAddr(pub usize);

impl PhysAddr {
    pub fn page_offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }

    pub fn floor(&self) -> PhysPageNum {
        PhysPageNum(self.0 / PAGE_SIZE)
    }

    pub fn ceil(&self) -> PhysPageNum {
        PhysPageNum((self.0 + PAGE_SIZE - 1) / PAGE_SIZE)
    }
}

impl From<usize> for PhysAddr {
    fn from(value: usize) -> Self {
        PhysAddr(value & ((1 << PHYS_ADDR_WIDTH) - 1))
    }
}

impl From<PhysPageNum> for PhysAddr {
    fn from(value: PhysPageNum) -> Self {
        Self(value.0 << PAGE_SIZE_BITS)
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct PhysPageNum(pub usize);

impl From<usize> for PhysPageNum {
    fn from(value: usize) -> Self {
        PhysPageNum(value & ((1 << PHYS_PAGE_NUMBER_WIDTH) - 1))
    }
}

impl TryFrom<PhysAddr> for PhysPageNum {
    type Error = KernelError;

    fn try_from(value: PhysAddr) -> Result<Self, Self::Error> {
        if value.page_offset() != 0 {
            return Err(
                KernelError::Conversion {
                    from: String::from("PhysAddr"),
                    to: String::from("PhysPageNum"),
                    reason: String::from("Page size not aligned."),
                }
            )
        }
        Ok(value.floor())
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct VirtAddr(pub usize);

impl From<usize> for VirtAddr {
    fn from(value: usize) -> Self {
        VirtAddr(value & ((1 << VIRT_ADDR_WIDTH) - 1))
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct VirtPageNum(pub usize);

impl From<usize> for VirtPageNum {
    fn from(value: usize) -> Self {
        VirtPageNum(value & ((1 << VIRT_PAGE_NUMBER_WIDTH) - 1))
    }
}
