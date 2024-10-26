MEMORY {
    RAM : ORIGIN = 0x80200000, LENGTH = 8M
}

OUTPUT_ARCH(riscv)
ENTRY(_entry)
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    . = BASE_ADDRESS;
    skernel = .;

    stext = .;
    .text : {
        *(.text.entry)
        . = ALIGN(4K);
        strampoline = .;
        *(.text.trampoline);
        . = ALIGN(4K);
        *(.text .text.*)
    } > RAM
    . = ALIGN(4K);
    etext = .;
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    } > RAM

    . = ALIGN(4K);
    erodata = .;
    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    } > RAM

    . = ALIGN(4K);
    edata = .;
    sbss_with_stack = .;
    .bss : {
      stack0_lower_bound = .;
    *(.bss.stack)
    stack0_top = .;
        /* *(.bss.stack) */
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    } > RAM

    . = ALIGN(4K);
    ebss = .;
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}