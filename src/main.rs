#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(naked_functions)]
#![feature(stmt_expr_attributes)]
#![test_runner(test_runner)]
extern crate alloc;
use core::ptr::addr_of;

use log::{debug, info};
use xv_r_kernel::{
    config::KERNEL_STACK_SIZE,
    logging,
    mm::{self, heap_allocator::init_heap, memory_set::remap_test},
    println, task, timer,
    trap::{self},
    KernelStack,
};
#[no_mangle]
#[link_section = ".bss.stack"]
pub static mut _KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};
// #[no_mangle]
// #[link_section = ".bss.user_stack"]
// pub static mut _USER_STACK: UserStack = UserStack {
//     data: [0; USER_STACK_SIZE],
// };

#[cfg(test)]
use xv_r_kernel::test::test_runner;
// global_asm!(include_str!("entry.asm"));

#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _entry() {
    unsafe {
        use core::arch::naked_asm;
        naked_asm!(".align 4", "la sp, stack0_top", "call _start",);
    }
}

/// # Safety
///
/// entry jumps here in machine mode on stack0.
#[no_mangle]
pub unsafe fn _start() {
    clear_bss();
    // println!("Hello, _start!");

    main()
}
fn clear_bss() {
    #[allow(unused)]
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
#[allow(unused)]
extern "C" {
    fn skernel();
    fn stext(); // begin addr of text segment
    fn etext(); // end addr of text segment
    fn srodata(); // start addr of Read-Only data segment
    fn erodata(); // end addr of Read-Only data segment
    fn sdata(); // start addr of data segment
    fn edata(); // end addr of data segment
    fn sbss(); // start addr of BSS segment
    fn ebss(); // end addr of BSS segment
    fn ekernel();
}

unsafe fn main() {
    logging::init();
    info!("[kernel] logging initialized.");

    init_heap();
    info!("[kernel] heap initialized.");
    mm::init();
    remap_test();

    info!("[kernel] memory management initialized.");

    assert_ne!(trap::init().address(), 0);
    // println!("Hello,{:#x}, sp __test!", sp);
    debug!(
        "[kernel] .kernel [{:#x}, {:#x})",
        skernel as usize, ekernel as usize
    );
    debug!(
        "[kernel] KERNEL_STACK_RANGE: {:#x} - {:#x}",
        addr_of!(_KERNEL_STACK) as usize,
        #[allow(static_mut_refs)]
        _KERNEL_STACK.get_sp()
    );
    println!("time: {}", riscv::register::time::read());
    trap::enable_timer_interrupt();
    timer::set_next_trigger();

    task::run_first_task();

    // debug!(
    //     "[kernel] USER_STACK_RANGE: {:#x} - {:#x}",
    //     addr_of!(_USER_STACK) as usize,
    //     _USER_STACK.get_sp()
    // );

    loop {}
}
