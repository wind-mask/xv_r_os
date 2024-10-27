use core::mem::MaybeUninit;

use crate::config::KERNEL_HEAP_SIZE;

// use buddy_system_allocator::LockedHeap;

// #[global_allocator]
// static HEAP_ALLOCATOR: LockedHeap<64> = LockedHeap::empty();

use linked_list_allocator::LockedHeap;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

// static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];
static mut KERNEL_HEAP_SPACE: MaybeUninit<[u8; KERNEL_HEAP_SIZE]> = MaybeUninit::uninit();

pub fn init_heap() {
    unsafe {
        #[allow(static_mut_refs)]
        HEAP_ALLOCATOR
            .lock()
            .init(KERNEL_HEAP_SPACE.as_mut_ptr() as *mut u8, KERNEL_HEAP_SIZE);
    }
}
#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}
