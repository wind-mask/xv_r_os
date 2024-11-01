use core::arch::{asm, global_asm};

use log::{debug, trace};
use riscv::register::{
    scause::{self, Trap},
    sie::set_stimer,
    stval,
    stvec::{self, Stvec, TrapMode},
};

use crate::{
    config::{TRAMPOLINE, TRAP_CONTEXT},
    proc::cpu::{current_trap_cx, current_user_token},
    syscall::syscall,
    task::{exit_current_and_run_next, suspend_current_and_run_next},
    timer::set_next_trigger,
};
pub mod context;
global_asm!(include_str!("./trap/trap.S"));
extern "C" {
    fn __alltraps();
    fn __restore();
}
#[inline(never)]
#[no_mangle]
/// 初始化trap处理
///
/// # Safety
///
/// 仅在内核初始化时调用一次
pub unsafe fn init() -> Stvec {
    debug!("__alltraps addr: {:#x}", __alltraps as usize);
    set_kernel_trap_entry();
    enable_timer_interrupt();
    stvec::read()
}
/// 设置内核态的trap处理入口
///
/// # Safety
///
/// 仅在进入内核态时调用
unsafe fn set_kernel_trap_entry() {
    stvec::write(trap_from_kernel as usize, TrapMode::Direct);
}
#[no_mangle]
#[repr(align(4))]
pub fn trap_from_kernel() -> ! {
    panic!("a trap from kernel!");
}
/// 设置用户态的trap处理入口
///
/// # Safety
///
/// 仅在内核态恢复用户态前调用
unsafe fn set_user_trap_entry() {
    stvec::write(TRAMPOLINE, TrapMode::Direct);
}

use riscv::interrupt::supervisor::Exception;
/// trap handler
///
/// # Safety
///
/// 作为中断处理函数调用，不手动调用
#[no_mangle]
pub unsafe fn trap_handler() -> ! {
    set_kernel_trap_entry();
    let cx = current_trap_cx();
    trace!("[kernel] Trap: {:#x}", cx.sepc);
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(const { Exception::UserEnvCall as usize }) => {
            trace!("[kernel] UserEnvCall");
            let mut cx = current_trap_cx();
            cx.sepc += 4;
            let result = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]);
            cx = current_trap_cx(); // cx may change after syscall
            cx.x[10] = result as usize;
        }
        Trap::Exception(const { Exception::StoreFault as usize })
        | Trap::Exception(const { Exception::StorePageFault as usize })
        | Trap::Exception(const { Exception::LoadFault as usize })
        | Trap::Exception(const { Exception::LoadPageFault as usize }) => {
            debug!("[kernel] PageFault in application, bad addr = {:#x}, bad instruction = {:#x}, kernel killed it.", stval, cx.sepc);
            exit_current_and_run_next(-2);
        }
        Trap::Exception(const { Exception::IllegalInstruction as usize }) => {
            debug!("[kernel] IllegalInstruction in application, kernel killed it.");
            exit_current_and_run_next(-3);
        }
        Trap::Interrupt(const { riscv::interrupt::Interrupt::SupervisorTimer as usize }) => {
            set_next_trigger();
            suspend_current_and_run_next();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }

    trap_return()
}

#[no_mangle]
/// set the new addr of __restore asm function in TRAMPOLINE page,
/// set the reg a0 = trap_cx_ptr, reg a1 = phy addr of usr page table,
/// finally, jump to new addr of __restore asm function
///
/// # Safety
///
/// 仅在内核态恢复用户态前调用
pub unsafe fn trap_return() -> ! {
    set_user_trap_entry();
    let trap_cx_ptr = TRAP_CONTEXT;
    let user_satp = current_user_token();
    let restore_va = __restore as usize - __alltraps as usize + TRAMPOLINE;
    asm!(
        "fence.i",
        "jr {restore_va}",             // jump to new addr of __restore asm function
        restore_va = in(reg) restore_va,
        in("a0") trap_cx_ptr,      // a0 = virt addr of Trap Context
        in("a1") user_satp,        // a1 = phy addr of usr page table
        options(noreturn)
    );
}
/// enable timer interrupt in sie CSR
///
/// # Safety
///
/// 仅在内核初始化时调用一次
unsafe fn enable_timer_interrupt() {
    set_stimer();
}
