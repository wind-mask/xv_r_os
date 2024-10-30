use alloc::vec::{Vec};
use log::trace;
use sbi_rt::Physical;

use crate::mm::page_table::translated_byte_buffer;
use crate::{printf::print, task::current_user_token};
const FD_STDIN: usize = 0;
const FD_STDOUT: usize = 1;
/// 功能：向文件描述符 fd 写入 len 字节的数据，数据来源于 buf 指向的缓冲区。
/// 返回值：返回实际写入的字节数，出错则返回 -1 。
/// syscall ID：64
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
/// 功能：从文件中读取一段内容到缓冲区。
/// 参数：fd 是待读取文件的文件描述符，切片 buffer 则给出缓冲区。
/// 返回值：如果出现了错误则返回 -1，否则返回实际读到的字节数。
/// syscall ID：63
pub fn sys_read(fd: usize, buffer: *const u8, len: usize) -> isize {
    trace!(
        "[kernel] sys_read: fd: {}, buffer: {:?}, len: {}",
        fd,
        buffer,
        len
    );
    match fd {
        FD_STDIN => {
            let mut buf = translated_byte_buffer(current_user_token(), buffer, len);
            use alloc::vec;
            let buffer = vec![0u8; len];
            let bufferr = Physical::new(
                buffer.len(),
                buffer.as_ptr() as u32 as usize,
                buffer.as_ptr() as usize >> 32,
            );
            let c = sbi_rt::console_read(bufferr);
            if c.is_err() {
                -1
            } else {
                buffer.iter().enumerate().for_each(|(i, &byte)| {
                    *buf[i] = byte;
                });
                c.value as isize
            }
        }
        _ => panic!("Unsupported fd in sys_read!"),
    }
}
