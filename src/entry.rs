use core::arch::global_asm;

global_asm!{
    "
.section .text.entry
.globl _start

.equ STACK_START, 0x80000000 + 0x4000

_start:
    li sp, STACK_START
    mv s0, x0
    j _start_rust
hang:
    wfi
    j hang
    "
}