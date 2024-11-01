use core::mem::MaybeUninit;

use crate::config::KERNEL_HEAP_SIZE;

// use buddy_system_allocator::LockedHeap;

// #[global_allocator]
// static HEAP_ALLOCATOR: LockedHeap<64> = LockedHeap::empty();

use linked_list_allocator::LockedHeap;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

#[link_section = ".bss.heap"]
static KERNEL_HEAP_SPACE: MaybeUninit<[u8; KERNEL_HEAP_SIZE]> = MaybeUninit::uninit();

/// 初始化堆空间
///
/// # Safety
///
/// 仅在内核内存初始化时调用，必须在alloc之前调用
pub unsafe fn init_heap() {
    HEAP_ALLOCATOR
        .lock()
        .init(&raw const KERNEL_HEAP_SPACE as *mut u8, KERNEL_HEAP_SIZE);
}
#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:?}", layout);
}
