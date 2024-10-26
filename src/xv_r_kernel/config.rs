pub const NCPU: usize = 8;
pub const BSER_STACK_SIZE: usize = 4096; // 4KB
pub const KERNEL_STACK_SIZE: usize = BSER_STACK_SIZE * NCPU;
pub const USER_STACK_SIZE: usize = BSER_STACK_SIZE;
pub const PAGE_SIZE_BITS: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SIZE_BITS;
pub const KERNEL_HEAP_SIZE: usize = 0x10_0000; // 1MB
pub const TRAMPOLINE: usize = usize::MAX - PAGE_SIZE + 1;
pub const TRAP_CONTEXT: usize = TRAMPOLINE - PAGE_SIZE;

/// Return (bottom, top) of a kernel stack in kernel space.
pub fn kernel_stack_position(app_id: usize) -> (usize, usize) {
    let top = TRAMPOLINE - app_id * (KERNEL_STACK_SIZE + PAGE_SIZE);
    let bottom = top - KERNEL_STACK_SIZE;
    (bottom, top)
}
