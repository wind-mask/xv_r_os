use core::mem::MaybeUninit;

const USER_HEAP_SIZE: usize = 0x8192;

#[global_allocator]
pub static ALLOCATOR: linked_list_allocator::LockedHeap =
    linked_list_allocator::LockedHeap::empty();
static mut USER_HEAP_SPACE: MaybeUninit<[u8; USER_HEAP_SIZE]> = MaybeUninit::uninit();
pub(crate) unsafe fn init_heap() {
    ALLOCATOR
        .lock()
        .init(&raw const USER_HEAP_SPACE as *mut u8, USER_HEAP_SIZE);
}
