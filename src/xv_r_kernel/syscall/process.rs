use crate::println;

pub fn sys_exit(exit_code: i32) -> ! {
    println!("exit with code {}", exit_code);
    loop {}
}
