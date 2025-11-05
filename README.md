# xv_r_os

一个用 Rust 编写的简单的 RISC-V 操作系统内核。

## 简介

`xv_r_os` 是一个基于 RISC-V 架构的教学目的操作系统。它采用类Unix设计，并实现了基本的内核功能，例如任务切换、系统调用和用户态程序加载。

## 功能特性

*   RISC-V 64 位架构支持
*   宏内核设计
*   用户态和内核态分离
*   基本的用户程序（如 `helloworld`, `shell`）
*   通过 SBI (Supervisor Binary Interface) 启动

## 环境要求

在开始之前，请确保您已安装以下依赖项：

*   **Rust 工具链**:
    *   确保您有最新的 `stable` 版 Rust。
    *   需要添加 `riscv64gc-unknown-none-elf` 目标：
        ```sh
        rustup target add riscv64gc-unknown-none-elf
        ```
*   **QEMU**: 用于模拟 RISC-V 硬件。需要 `qemu-system-riscv64`。
*   **Just**: 一个方便的命令运行器，用于执行项目脚本。
    ```sh
    cargo install just
    ```

## 构建和运行

本项目使用 `just` 来简化构建和运行流程。

1.  **构建所有组件 (内核和用户程序):**
    ```sh
    just build
    ```

2.  **在 QEMU 中运行:**
    ```sh
    just run
    ```

3.  **调试:**
    打开一个终端并运行：
    ```sh
    just debug
    ```

4.  **清理构建产物:**
    ```sh
    just clean
    ```

## 项目结构

```
.
├── bootloader/     # SBI 二进制文件
├── src/            # 内核和主程序源码
│   ├── xv_r_kernel/  # 内核库
│   └── main.rs     # 内核入口
├── user/           # 用户态应用程序
├── .justfile       # 项目命令脚本
└── Cargo.toml      # 项目和工作区配置
```


