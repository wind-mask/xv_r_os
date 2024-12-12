#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![feature(linkage)]
#![feature(alloc_error_handler)]
pub mod console;
mod heap;
mod lang_items;
mod syscall;
pub mod test;

use heap::init_heap;
use syscall::*;
#[cfg(test)]
use test::test_runner;
/// 用户程序汇编级入口点，由链接器设为entry，需要`extern "C"`,`#[no_mangle]`
///
/// # Safety
///
/// 由链接器设为entry，不应该被用户调用
#[no_mangle]
#[link_section = ".text.entry"]
unsafe extern "C" fn _start() -> ! {
    clear_bss();
    init_heap();
    exit(main());

    panic!("unreachable after sys_exit!")
}
/// 初始化bss段
///
/// # Safety
///
/// 仅在_start中调用
unsafe fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|addr| {
        (addr as *mut u8).write_volatile(0);
    });
}
#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}

pub fn read(fd: usize, buf: &mut [u8]) -> isize {
    sys_read(fd, buf)
}

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}
pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}
pub fn wait(exit_code: &mut i32) -> isize {
    loop {
        match sys_waitpid(-1, exit_code as *mut _) {
            -2 => {
                yield_();
            }
            // -1 or a real pid
            exit_pid => return exit_pid,
        }
    }
}

pub fn waitpid(pid: usize, exit_code: &mut i32) -> isize {
    loop {
        match sys_waitpid(pid as isize, exit_code as *mut _) {
            -2 => {
                yield_();
            }
            // -1 or a real pid
            exit_pid => return exit_pid,
        }
    }
}
pub fn exec(path: &str) -> isize {
    sys_exec(path)
}
pub fn fork() -> isize {
    sys_fork()
}
pub fn yield_() -> isize {
    sys_yield()
}
