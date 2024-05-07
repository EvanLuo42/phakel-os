use alloc::vec::Vec;
use core::fmt;
use core::fmt::{Debug, Formatter, Write};

use lazy_static::lazy_static;
use spin::Mutex;

use crate::config::MEMORY_END;
use crate::errors::KernelError;
use crate::mm::address::{PhysAddr, PhysPageNum};
use crate::symbols;

trait FrameAllocator {
    fn new() -> Self;
    fn alloc(&mut self) -> Option<PhysPageNum>;
    fn dealloc(&mut self, ppn: PhysPageNum) -> Result<(), KernelError>;
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

pub struct FrameTracker {
    pub ppn: PhysPageNum
}

impl FrameTracker {
    pub fn new(ppn: PhysPageNum) -> FrameTracker {
        let bytes_array = ppn.get_bytes_array();
        for i in bytes_array {
            *i = 0;
        }
        Self { ppn }
    }
}

impl Debug for FrameTracker {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("FrameTracker:PPN={:#x}", self.ppn.0))
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        frame_dealloc(self.ppn);
    }
}

lazy_static! {
    pub static ref FRAME_ALLOCATOR: Mutex<StackFrameAllocator> = 
        Mutex::new(StackFrameAllocator::new());
}

pub fn init_frame_allocator() {
    FRAME_ALLOCATOR.lock().init(
        PhysAddr::from(symbols::ekernel as usize).ceil(),
        PhysAddr::from(MEMORY_END).floor()
    );
}

pub fn frame_alloc() -> Option<FrameTracker> {
    FRAME_ALLOCATOR
        .lock()
        .alloc()
        .map(FrameTracker::new)
}

pub fn frame_dealloc(ppn: PhysPageNum) -> Result<(), KernelError> {
    FRAME_ALLOCATOR.lock().dealloc(ppn)
}

pub fn frame_allocator_test() {
    let mut v: Vec<FrameTracker> = Vec::new();
    for i in 0..5 {
        let frame = frame_alloc().unwrap();
        println!("{:?}", frame);
        v.push(frame);
    }
    v.clear();
    for i in 0..5 {
        let frame = frame_alloc().unwrap();
        println!("{:?}", frame);
        v.push(frame);
    }
    drop(v);
    println!("frame_allocator_test passed!");
}