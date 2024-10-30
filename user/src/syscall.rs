use core::arch::asm;

use num_enum::TryFromPrimitive;

// use xv_r_kernel::syscall::SyscallId;
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
pub fn syscall(id: SyscallId, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id as usize,
        );
    }
    ret
}
pub fn sys_read(fd: usize, buffer: &mut [u8]) -> isize {
    syscall(
        SyscallId::Read,
        [fd, buffer.as_mut_ptr() as usize, buffer.len()],
    )
}
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(
        SyscallId::Write,
        [fd, buffer.as_ptr() as usize, buffer.len()],
    )
}

pub fn sys_exit(exit_code: i32) -> isize {
    syscall(SyscallId::Exit, [exit_code as usize, 0, 0])
}
pub fn sys_exec(path: &str) -> isize {
    syscall(SyscallId::Exec, [path.as_ptr() as usize, 0, 0])
}
pub fn sys_fork() -> isize {
    syscall(SyscallId::Fork, [0, 0, 0])
}
pub fn sys_yield() -> isize {
    syscall(SyscallId::Yield, [0, 0, 0])
}
pub fn sys_waitpid(pid: isize, exit_code: *mut i32) -> isize {
    syscall(SyscallId::Waitpid, [pid as usize, exit_code as usize, 0])
}
