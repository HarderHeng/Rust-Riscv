# entry.s - RISC-V 64 裸机启动代码
.section .text.entry
.globl _start

# 假设你的 RAM 从 0x8000_0000 开始，栈从高地址向下增长
.equ STACK_START, 0x80000000 + 0x4000  # 16KB 栈空间

_start:
    # 1. 初始化栈指针 sp
    li sp, STACK_START

    # 2. 清除 s0（帧指针）
    mv s0, x0

    # 3. 跳转到 Rust 入口（即你现有的 _start）
    j _start_rust

# 无限循环，防止返回
hang:
    wfi
    j hang