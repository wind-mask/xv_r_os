use alloc::vec::Vec;
use log::trace;

use crate::mm::page_table::translated_byte_buffer;
use crate::{print, task::current_user_token};
const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    trace!("[kernel] sys_write: fd: {}, len: {}", fd, len);
    match fd {
        FD_STDOUT => {
            let buffer = translated_byte_buffer(current_user_token(), buf, len);
            let buffer: Vec<u8> = buffer.into_iter().map(|&mut byte| byte).collect();

            print!("{}", core::str::from_utf8(&buffer).unwrap());
            len as isize
        }
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}
