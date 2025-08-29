#![no_std]
#![no_main]

mod entry;
mod uart;
mod plic;
pub mod utils;
mod arch{
    pub mod riscv;
}

use core::panic::PanicInfo;
use uart::*;
use plic::*;

#[unsafe(no_mangle)]
pub fn _start_rust() -> !{
    uart_init();
    uart_print(r_mhartid!());
    uart_print("Hello Rust On Riscv of Qemu!");
    loop{
        
    };
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    uart_print("Panic!");
    loop{};
}