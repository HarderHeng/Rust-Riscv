#![no_std]
#![no_main]

mod entry;
mod uart;

use core::panic::PanicInfo;
use uart::*;
#[unsafe(no_mangle)]
pub fn _start_rust() -> !{
    uart_init();
    uart_print("Hello Rust On Riscv of Qemu!");
    loop{
        
    };
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{};
}