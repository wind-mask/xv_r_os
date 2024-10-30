use fs::{sys_read, sys_write};
use log::trace;
use num_enum::TryFromPrimitive;
use process::{sys_exit, sys_waitpid, sys_yield};
use time::sys_get_time;
mod fs;
mod process;
mod time;

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
            SyscallId::Fork => todo!(),
            SyscallId::Exec => todo!(),
        },
        Err(e) => {
            panic!("Unsupported syscall_id: {}", e.number);
        }
    }
}
