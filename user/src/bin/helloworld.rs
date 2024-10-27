#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#[cfg(test)]
use user_lib::test::test_runner;
use user_lib::print;

#[no_mangle]
fn main() {
    print!("Hello, world!\n");
    loop {}
}
