use core::arch::asm;

use xv_r_kernel_pub::SyscallId;

/// 调用系统调用
///
/// # Safety
///
/// 确保有效的系统调用号和参数
pub unsafe fn syscall(id: SyscallId, args: [usize; 3]) -> isize {
    let mut ret: isize;
    asm!(
        "ecall",
        inlateout("x10") args[0] => ret,
        in("x11") args[1],
        in("x12") args[2],
        in("x17") id as usize,
    );
    ret
}
// UNSAFE: 包装有效的syscall
pub fn sys_read(fd: usize, buffer: &mut [u8]) -> isize {
    unsafe {
        syscall(
            SyscallId::Read,
            [fd, buffer.as_mut_ptr() as usize, buffer.len()],
        )
    }
}
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    unsafe {
        syscall(
            SyscallId::Write,
            [fd, buffer.as_ptr() as usize, buffer.len()],
        )
    }
}

pub fn sys_exit(exit_code: i32) -> isize {
    unsafe { syscall(SyscallId::Exit, [exit_code as usize, 0, 0]) }
}
pub fn sys_exec(path: &str) -> isize {
    unsafe { syscall(SyscallId::Exec, [path.as_ptr() as usize, 0, 0]) }
}
pub fn sys_fork() -> isize {
    unsafe { syscall(SyscallId::Fork, [0, 0, 0]) }
}
pub fn sys_yield() -> isize {
    unsafe { syscall(SyscallId::Yield, [0, 0, 0]) }
}
pub fn sys_waitpid(pid: isize, exit_code: *mut i32) -> isize {
    unsafe { syscall(SyscallId::Waitpid, [pid as usize, exit_code as usize, 0]) }
}
