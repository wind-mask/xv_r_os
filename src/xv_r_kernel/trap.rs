use core::arch::asm;

use context::TrapContext;
use log::{debug, info, trace};
use riscv::register::{
    scause::{self, Exception, Trap},
    stval,
    stvec::{self, Stvec, TrapMode},
};

use crate::{
    config::{TRAMPOLINE, TRAP_CONTEXT},
    syscall::syscall,
    task::current_user_token,
};
pub mod context;

#[inline(never)]
#[no_mangle]
pub fn init() -> Stvec {
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
    stvec::read()
}
// pub fn trap_from_kernel() -> ! {
//     panic!("a trap from kernel!");
// }
#[naked]
#[link_section = ".text.trampoline"]
pub extern "C" fn __alltraps() -> ! {
    use core::arch::naked_asm;
    unsafe {
        naked_asm!(
            ".align 4",
            "csrrw sp, sscratch, sp",
            // "addi sp, sp, -34*8",
            "sd x1, 1*8(sp)",
            "sd x3, 3*8(sp)",
            "sd x5, 5*8(sp)",
            "sd x6, 6*8(sp)",
            "sd x7, 7*8(sp)",
            "sd x8, 8*8(sp)",
            "sd x9, 9*8(sp)",
            "sd x10, 10*8(sp)",
            "sd x11, 11*8(sp)",
            "sd x12, 12*8(sp)",
            "sd x13, 13*8(sp)",
            "sd x14, 14*8(sp)",
            "sd x15, 15*8(sp)",
            "sd x16, 16*8(sp)",
            "sd x17, 17*8(sp)",
            "sd x18, 18*8(sp)",
            "sd x19, 19*8(sp)",
            "sd x20, 20*8(sp)",
            "sd x21, 21*8(sp)",
            "sd x22, 22*8(sp)",
            "sd x23, 23*8(sp)",
            "sd x24, 24*8(sp)",
            "sd x25, 25*8(sp)",
            "sd x26, 26*8(sp)",
            "sd x27, 27*8(sp)",
            "sd x28, 28*8(sp)",
            "sd x29, 29*8(sp)",
            "sd x30, 30*8(sp)",
            "sd x31, 31*8(sp)",
            "csrr t0, sstatus",
            "csrr t1, sepc",
            "sd t0, 32*8(sp)",
            "sd t1, 33*8(sp)",
            "csrr t2, sscratch",
            "sd t2, 2*8(sp)",
            "ld t0,34*8(sp)",
            "ld t1, 36*8(sp)",
            "ld sp,35*8(sp)",
            // "mv a0, sp",
            "csrw satp, t0",
            "sfence.vma",
            "jr t1",
            "call __restore",
        );
    }
}

#[naked]
#[no_mangle]
#[link_section = ".text.trampoline"]
/// # Safety
///
/// only used in trap handler
pub unsafe extern "C" fn __restore(cx_addr: usize) {
    unsafe {
        use core::arch::naked_asm;
        naked_asm!(
            ".align 4",
            "mv sp, a0",
            "ld t0, 32*8(sp)",
            "ld t1, 33*8(sp)",
            "ld t2, 2*8(sp)",
            "csrw sstatus, t0",
            "csrw sepc, t1",
            "csrw sscratch, t2", // sscratch now is user stack pointer
            "ld x1, 1*8(sp)",
            "ld x3, 3*8(sp)",
            "ld x5, 5*8(sp)",
            "ld x6, 6*8(sp)",
            "ld x7, 7*8(sp)",
            "ld x8, 8*8(sp)",
            "ld x9, 9*8(sp)",
            "ld x10, 10*8(sp)",
            "ld x11, 11*8(sp)",
            "ld x12, 12*8(sp)",
            "ld x13, 13*8(sp)",
            "ld x14, 14*8(sp)",
            "ld x15, 15*8(sp)",
            "ld x16, 16*8(sp)",
            "ld x17, 17*8(sp)",
            "ld x18, 18*8(sp)",
            "ld x19, 19*8(sp)",
            "ld x20, 20*8(sp)",
            "ld x21, 21*8(sp)",
            "ld x22, 22*8(sp)",
            "ld x23, 23*8(sp)",
            "ld x24, 24*8(sp)",
            "ld x25, 25*8(sp)",
            "ld x26, 26*8(sp)",
            "ld x27, 27*8(sp)",
            "ld x28, 28*8(sp)",
            "ld x29, 29*8(sp)",
            "ld x30, 30*8(sp)",
            "ld x31, 31*8(sp)",
            "addi sp, sp, 34*8",
            "csrrw sp, sscratch, sp",
            "sret",
        );
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    trace!("[kernel] Trap: {:#x}", cx.sepc);
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            debug!("[kernel] UserEnvCall");
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            info!("[kernel] PageFault in application, kernel killed it.");
            // run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            info!("[kernel] IllegalInstruction in application, kernel killed it.");
            // run_next_app();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    cx
}
fn set_user_trap_entry() {
    unsafe {
        stvec::write(TRAMPOLINE, TrapMode::Direct);
    }
}
#[no_mangle]
/// set the new addr of __restore asm function in TRAMPOLINE page,
/// set the reg a0 = trap_cx_ptr, reg a1 = phy addr of usr page table,
/// finally, jump to new addr of __restore asm function
pub fn trap_return() -> ! {
    set_user_trap_entry();
    let trap_cx_ptr = TRAP_CONTEXT;
    let user_satp = current_user_token();
    // extern "C" {
    //     fn __alltraps();
    //     fn __restore();
    // }
    let restore_va = __restore as usize - __alltraps as usize + TRAMPOLINE;
    unsafe {
        asm!(
            "fence.i",
            "jr {restore_va}",             // jump to new addr of __restore asm function
            restore_va = in(reg) restore_va,
            in("a0") trap_cx_ptr,      // a0 = virt addr of Trap Context
            in("a1") user_satp,        // a1 = phy addr of usr page table
            options(noreturn)
        );
    }
}
