use alloc::string::String;
use bitflags::bitflags;
use crate::errors::KernelError;
use crate::mm::address::PhysPageNum;

bitflags! {
    #[derive(PartialEq, Eq)]
    pub struct PTEFlags: u8 {
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PageTableEntry {
    pub bits: usize
}

impl PageTableEntry {
    pub fn new(ppn: PhysPageNum, flags: PTEFlags) -> PageTableEntry {
        PageTableEntry {
            bits: ppn.0 << 10 | flags.bits() as usize
        }
    }

    pub fn empty() -> Self {
        PageTableEntry {
            bits: 0
        }
    }

    pub fn ppn(&self) -> PhysPageNum {
        (self.bits >> 10 & ((1usize << 44) - 1)).into()
    }

    pub fn flags(&self) -> Result<PTEFlags, KernelError> {
        PTEFlags::from_bits(self.bits as u8).ok_or(KernelError::Conversion {
            from: String::from("PageTableEntry.bits"),
            to: String::from("PTEFlags"),
            reason: String::from("Unknown bits found."),
        })
    }

    pub fn is_valid(&self) -> Result<bool, KernelError> {
        Ok((self.flags()? & PTEFlags::V) != PTEFlags::empty())
    }
}