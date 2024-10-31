pub const NUM_APP: usize = 2;
pub static APP_0_DATA: &[u8] =
    include_bytes!("../../target/riscv64gc-unknown-none-elf/release/helloworld");
pub static APP_1_DATA: &[u8] = include_bytes!("../../target/riscv64gc-unknown-none-elf/release/shell");
