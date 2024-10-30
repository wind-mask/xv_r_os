    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad 2
    .quad app_0_start
    .quad app_1_start
    .quad app_2_start
    # .quad app_1_end

    .section .data
    .global app_0_start
    .global app_1_start
    .global app_2_start
    # .global app_1_end
    .align 3
app_0_start:
    .incbin "target/riscv64gc-unknown-none-elf/release/helloworld"
app_1_start:
    .incbin "target/riscv64gc-unknown-none-elf/release/shell"
app_2_start:
    .incbin "target/riscv64gc-unknown-none-elf/release/init"
# app_1_end: