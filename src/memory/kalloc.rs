use core::alloc::Layout;
use buddy_system_allocator::LockedHeap;
use crate::config::KERNEL_HEAP_SIZE;

#[global_allocator]
pub static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("Allocation error: {:?}", layout);
}

pub fn init_heap() {
    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE)
    }
}
