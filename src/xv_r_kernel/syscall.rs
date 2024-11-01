use fs::{sys_read, sys_write};
use log::trace;
use num_enum::TryFromPrimitive;
use process::{sys_exec, sys_exit, sys_fork, sys_waitpid, sys_yield};
use time::sys_get_time;
mod fs;
mod process;
mod time;

use xv_r_kernel_pub::SyscallId;
pub(crate) fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    trace!(
        "[kernel] syscall_id: {}, args: [{:#x}, {:#x}, {:#x}]",
        syscall_id,
        args[0],
        args[1],
        args[2]
    );
    match SyscallId::try_from_primitive(syscall_id) {
        Ok(id) => match id {
            SyscallId::Read => sys_read(args[0], args[1] as *const u8, args[2]),
            SyscallId::Write => sys_write(args[0], args[1] as *const u8, args[2]),
            SyscallId::Exit => sys_exit(args[0] as i32),
            SyscallId::Time => sys_get_time(args[0] as *mut time::TimeVal, args[1]),
            SyscallId::Yield => sys_yield(),
            SyscallId::Waitpid => sys_waitpid(args[0] as isize, args[1] as *mut i32),
            SyscallId::Fork => sys_fork(),
            SyscallId::Exec => sys_exec(args[0] as *const u8),
        },
        Err(e) => {
            panic!("Unsupported syscall_id: {}", e.number);
        }
    }
}
