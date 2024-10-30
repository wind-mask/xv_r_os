use core::fmt::{self, Write};

struct Stdout;

const STDOUT: usize = 1;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(STDOUT, s.as_bytes());

        Ok(())
    }
}

pub fn __print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! user_print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::user::__print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! user_println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::user::__print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}
pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}

// const SYSCALL_WRITE: usize = 64;
// const SYSCALL_EXIT: usize = 93;

use core::arch::{asm, global_asm};

use crate::syscall::{SYSCALL_EXIT, SYSCALL_WRITE};
pub fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        );
    }

    ret
}
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(exit_code: i32) -> isize {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
}
