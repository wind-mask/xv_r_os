use fs::sys_write;
use log::trace;
use process::sys_exit;
use time::sys_get_time;
mod fs;
mod process;
mod time;
pub const SYSCALL_WRITE: usize = 64;
pub const SYSCALL_EXIT: usize = 93;
pub const SYSCALL_TIME: usize = 169;
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    trace!(
        "[kernel] syscall_id: {:}, args: [{:#x}, {:#x}, {:#x}]",
        syscall_id,
        args[0],
        args[1],
        args[2]
    );
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_TIME => sys_get_time(args[0] as *mut time::TimeVal, args[1]),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
