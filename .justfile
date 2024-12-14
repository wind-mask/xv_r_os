# 使用nu shell跨平台
set shell := ["nu", "-c"]
# 加载.env文件
set dotenv-load := true
# 默认只是列出所有的recipe
default:
    @just --list --unsorted --justfile {{justfile()}}

#BOOTLOADER := "./bootloader/rustsbi-qemu.bin"
BOOTLOADER := "./bootloader/rustsbi-qemu-release/rustsbi-qemu.bin"
KERNEL_BIN := "./os_bin/xv_r_os.bin"
KERNEL_BIN_DEBUG := "./os_bin/xv_r_os-debug.bin"
KERNEL_ENTRY_PA := "0x80200000"
# QEMU_ARGS := "-machine virt  -bios "  + BOOTLOADER +  " -device loader,file="+KERNEL_BIN+",addr="+KERNEL_ENTRY_PA + " -m 8M" 
QEMU_ARGS := "-machine virt -bios "+ BOOTLOADER  +  " -kernel " + KERNEL_BIN + " -m 8M" 
QEMU_ARGS_DEBUG := "-machine virt  -bios "  + BOOTLOADER +  " -device loader,file="+KERNEL_BIN_DEBUG+",addr="+KERNEL_ENTRY_PA + " -m 8M"
#QEMU_ARGS := "-machine virt  -bios "  + BOOTLOADER + " -kernel " + KERNEL_BIN
LOG_LEVEL_ENV := env_var_or_default('LOG', 'info')
build LOG_LEVEL = LOG_LEVEL_ENV:
    @echo "LOG_LEVEL: {{LOG_LEVEL}}"
    @echo "build user program"
    @cd user;cargo +nightly build --release -v
    @echo "cargo build"
    @$env.LOG = '{{LOG_LEVEL}}' ;cargo +nightly build --release -v
    # @cargo +nightly build --release -v
build-debug LOG_LEVEL = LOG_LEVEL_ENV:
    @echo "LOG_LEVEL: {{LOG_LEVEL}}"
    @echo "build user program"
    @cd user;cargo +nightly build -v
    @echo "cargo build"
    @$env.LOG = '{{LOG_LEVEL}}' ;cargo +nightly build -v
    # @cargo +nightly build -v
objcopy LOG_LEVEL = LOG_LEVEL_ENV: (build LOG_LEVEL)  
    @echo "cargo objcopy"
    @$env.LOG = '{{LOG_LEVEL}}'; cargo +nightly  objcopy --release -v --bin xv_r_os -- --strip-all -O binary os_bin/xv_r_os.bin

objcopy-debug LOG_LEVEL = LOG_LEVEL_ENV: (build-debug LOG_LEVEL)
    @echo "cargo objcopy"
    @$env.LOG = '{{LOG_LEVEL}}'; cargo +nightly  objcopy -v --bin xv_r_os -- --strip-all -O binary os_bin/xv_r_os-debug.bin
qemu LOG_LEVEL = LOG_LEVEL_ENV: (objcopy LOG_LEVEL)
    @echo "qemu-system-riscv64 {{QEMU_ARGS}}"
    @qemu-system-riscv64 {{QEMU_ARGS}}

qemu-console LOG_LEVEL = LOG_LEVEL_ENV: (objcopy LOG_LEVEL)
    @echo "qemu-system-riscv64 {{QEMU_ARGS}} -nographic"
    @qemu-system-riscv64 {{QEMU_ARGS}} -nographic

qemu-debug LOG_LEVEL = LOG_LEVEL_ENV: (objcopy-debug LOG_LEVEL)
    @echo "qemu-system-riscv64 {{QEMU_ARGS_DEBUG}} -nographic -s -S"
    @qemu-system-riscv64 {{QEMU_ARGS_DEBUG}}  -nographic -s -S
clean:
    @echo "cargo clean"
    @cargo clean
    @rm -rf os_bin

raw_qemu target:
    @riscv64-unknown-elf-objcopy {{target}} --strip-all -O binary os_bin/xv_r_os.bin
    @qemu-system-riscv64 {{QEMU_ARGS}}
