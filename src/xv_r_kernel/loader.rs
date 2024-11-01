pub const NUM_APP: usize = 2;
pub static APP_HELLOWORLD_DATA: &[u8] =
    include_bytes!("../../target/riscv64gc-unknown-none-elf/release/helloworld");
pub static APP_SHELL_DATA: &[u8] =
    include_bytes!("../../target/riscv64gc-unknown-none-elf/release/shell");
pub static APP_INIT_DATA: &[u8] =
    include_bytes!("../../target/riscv64gc-unknown-none-elf/release/init");
pub fn get_app_data_by_name(name: &str) -> Option<&[u8]> {
    if name == "helloworld" {
        Some(APP_HELLOWORLD_DATA)
    } else if name == "shell" {
        Some(APP_SHELL_DATA)
    } else if name == "init" {
        Some(APP_INIT_DATA)
    } else {
        None
    }
}
