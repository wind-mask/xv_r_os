#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]

#[cfg(test)]
use user_lib::test::test_runner;
use user_lib::{exec, fork, println, wait, yield_};

#[no_mangle]
fn main() {
    if fork() == 0 {
        exec("shell\0");
    } else {
        loop {
            let mut exit_code = 0;
            let pid = wait(&mut exit_code);
            if pid == -1 {
                yield_();
                continue;
            }
            println!("Process {} exited with code {}", pid, exit_code);
        }
    }
}
