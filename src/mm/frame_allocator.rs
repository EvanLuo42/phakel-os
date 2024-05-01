use alloc::vec::Vec;

use crate::errors::KernelError;
use crate::mm::address::PhysPageNum;

trait FrameAllocator {
    fn new() -> Self;
    fn alloc(&mut self) -> Option<PhysPageNum>;
    fn dealloc(&mut self, ppn: PhysPageNum);
}

pub struct StackFrameAllocator {
    current: usize,
    end: usize,
    recycled: Vec<usize>
}

impl StackFrameAllocator {
    pub fn init(&mut self, left_ppn: PhysPageNum, right_ppn: PhysPageNum) {
        self.current = left_ppn.0;
        self.end = right_ppn.0;
    }

    pub fn has_remain(&self) -> bool {
        self.current != self.end
    }

    pub fn is_allocated(&self, ppn: PhysPageNum) -> bool {
        let ppn = ppn.0;
        ppn >= self.current || self.recycled.iter().any(|&x| x == ppn)
    }
}

impl FrameAllocator for StackFrameAllocator {
    fn new() -> Self {
        StackFrameAllocator {
            current: 0,
            end: 0,
            recycled: Vec::new(),
        }
    }

    fn alloc(&mut self) -> Option<PhysPageNum> {
        self.recycled.pop()
            .map(|x| x.into())
            .or_else(|| {
                if !self.has_remain() {
                    return None
                }
                self.current += 1;
                Some((self.current - 1).into())
            })
    }

    fn dealloc(&mut self, ppn: PhysPageNum) -> Result<(), KernelError> {
        if self.is_allocated(ppn) {
            return Err(KernelError::Deallocate { ppn: ppn.0 })
        }
        self.recycled.push(ppn.0);
        Ok(())
    }
}