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
    proc::cpu::run_tasks,
    timer,
    trap::{self},
    KernelStack,
};
#[no_mangle]
#[link_section = ".bss.stack"]
static _KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};

#[cfg(test)]
use xv_r_kernel::test::test_runner;

#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
/// 汇编级入口点
///
/// # Safety
///
/// 由链接器设为entry，不应该被用户调用
pub unsafe extern "C" fn _entry() {
    unsafe {
        use core::arch::naked_asm;
        naked_asm!(".align 4", "la sp, stack0_top", "call _start",);
    }
}

/// Rust级入口点,由内联汇编调用，因此 `#[no_mangle]`
/// # Safety
///
/// 由_entry调用，不应该被用户调用
#[no_mangle]
pub unsafe fn _start() {
    clear_bss();
    // println!("Hello, _start!");

    main()
}
/// 初始化bss段
/// 1. sbss和ebss是由链接器脚本公开
/// 2. bss段是未初始化的全局变量段
/// 3. bss段的起始地址是sbss，结束地址是ebss
/// 4. bss段的内容初始化为0
///
/// # Safety
///
/// 只允许在启动时调用一次
unsafe fn clear_bss() {
    #[allow(unused)]
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| (a as *mut u8).write_volatile(0));
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

/// 主函数
///
/// 由_start在RUST中调用，安全入口
fn main() {
    // 初始化日志系统
    logging::init();
    info!("[kernel] logging initialized.");
    // 初始化堆，必须先于alloc之前调用
    unsafe { init_heap() };
    info!("[kernel] heap initialized.");
    // 初始化内存管理系统，分页，虚拟内存
    unsafe { mm::init() };
    remap_test();

    info!("[kernel] memory management initialized.");
    // UNSAFE: unsafe：trap::init()只能在内核初始化时调用一次
    assert_ne!(unsafe { trap::init().address() }, 0);
    debug!(
        "[kernel] .kernel [{:#x}, {:#x})",
        skernel as usize, ekernel as usize
    );
    debug!(
        "[kernel] KERNEL_STACK_RANGE: {:#x} - {:#x}",
        addr_of!(_KERNEL_STACK) as usize,
        _KERNEL_STACK.get_sp()
    );
    info!("time: {}", riscv::register::time::read());
    // trap::enable_timer_interrupt();
    timer::set_next_trigger();

    unsafe { run_tasks() };
    unreachable!()
}
