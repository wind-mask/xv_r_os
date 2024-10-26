use core::arch::naked_asm;

use super::TaskContext;

/// Switch to the context of `next_task_cx_ptr`, saving the current context
/// in `current_task_cx_ptr`.
#[naked]
#[no_mangle]
pub unsafe extern "C" fn __switch(
    current_task_cx_ptr: *mut TaskContext,
    next_task_cx_ptr: *const TaskContext,
) {
    naked_asm!(
        ".align 4",
        "sd sp,8(a0)",
        "sd ra,0(a0)",
        "sd s0,16(a0)",
        "sd s1,24(sp)",
        "sd s2,32(sp)",
        "sd s3,40(sp)",
        "sd s4,48(sp)",
        "sd s5,56(sp)",
        "sd s6,64(sp)",
        "sd s7,72(sp)",
        "sd s8,80(sp)",
        "sd s9,88(sp)",
        "sd s10,96(sp)",
        "sd s11,104(sp)",
        "ld ra,0(a1)",
        "ld s0,16(a1)",
        "ld s1,24(a1)",
        "ld s2,32(a1)",
        "ld s3,40(a1)",
        "ld s4,48(a1)",
        "ld s5,56(a1)",
        "ld s6,64(a1)",
        "ld s7,72(a1)",
        "ld s8,80(a1)",
        "ld s9,88(a1)",
        "ld s10,96(a1)",
        "ld s11,104(a1)",
        "ld sp,8(a1)",
        "ret",
    )
}
