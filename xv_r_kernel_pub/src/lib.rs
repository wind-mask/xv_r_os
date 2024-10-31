#![no_std]
// #![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]

use num_enum::TryFromPrimitive;
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
    }
}
#[derive(TryFromPrimitive)]
#[repr(usize)]
pub enum SyscallId {
    Read = 63,
    Write = 64,
    Exit = 93,
    Yield = 124,
    Time = 169,
    Fork = 220,
    Exec = 221,
    Waitpid = 260,
}
