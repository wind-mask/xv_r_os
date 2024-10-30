
OUTPUT_ARCH(riscv)
ENTRY(_start)

BASE_ADDRESS = 0x10000;

SECTIONS
{
    . = BASE_ADDRESS;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }
    . = ALIGN(4K);
    .rodata : {
        *(.rodata .rodata.*)
    }
    . = ALIGN(4K);
    .data : {
        *(.data .data.*)
    }
    . = ALIGN(4K);
    sbss = .;
    .sbss : {
        *(.sbss .sbss.*)
    }
        . = ALIGN(4K);
    .bss : {
        *(.bss .bss.*)
    }
    ebss = .;
    /DISCARD/ : {
        *(.eh_frame)
        *(.debug*)
    }
}