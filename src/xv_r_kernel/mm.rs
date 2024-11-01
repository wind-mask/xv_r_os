use log::debug;
use memory_set::KERNEL_SPACE;

pub mod address;
mod frame_allocator;
pub mod heap_allocator;
pub mod memory_set;
pub mod page_table;

/// initiate heap allocator, frame allocator and kernel space
///
/// # Safety
///
/// only called once when kernel is initialized
pub unsafe fn init() {
    heap_allocator::init_heap();
    debug!("[kernel] heap allocator initialized.");
    frame_allocator::init_frame_allocator();
    // frame_allocator_test();
    debug!("[kernel] frame allocator initialized.");
    KERNEL_SPACE.lock().activate();
    debug!("[kernel] kernel space activated.");
}
