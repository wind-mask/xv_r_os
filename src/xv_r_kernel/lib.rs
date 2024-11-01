#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![feature(naked_functions)]
#![feature(fn_align)]
#![feature(alloc_error_handler)]
#![feature(inline_const_pat)]

use config::{KERNEL_STACK_SIZE, USER_STACK_SIZE};
extern crate alloc;
#[cfg(test)]
use crate::test::test_runner;
pub mod board;
pub mod config;
mod hal;
pub mod loader;
pub mod logging;
pub mod mm;
mod panic;
pub mod proc;
// mod sync;
pub mod syscall;
pub mod task;
pub mod test;
pub mod timer;
pub mod trap;
// pub mod user;
// #[macro_use]
pub(crate) mod printf;

#[repr(C)]
#[repr(align(4096))]
pub struct KernelStack {
    pub data: [u8; KERNEL_STACK_SIZE],
}
#[repr(C)]
#[repr(align(4096))]
pub struct UserStack {
    pub data: [u8; USER_STACK_SIZE],
}
impl KernelStack {
    pub fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
}
impl UserStack {
    pub fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}
