#![no_std]
#![no_main]

mod entry;

use core::ptr;
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub fn _start_rust() -> !{
    const UART0:i32         =   0x10000000;
    const THR:i32           =   0;
    const IER:i32           =   1;
    const _ISR:i32           =   2;
    const LCR:i32           =   3;
    const FCR:i32           =   2;
    const LCR_BAUD_LATCH:u8 =   1<<7;
    const LCR_EIGHT_BITS:u8 =   3<<0;
    const FCR_FIFO_ENABLE:u8=   1<<0;
    const FCR_FIFO_CLEAR:u8 =   3<<1;
    const IER_RX_ENABLE:u8  =   1<<0;
    const IER_TX_ENABLE:u8  =   1<<1;
    unsafe {
        let mut ptr = (UART0 + IER) as *mut u8;
        ptr.write_volatile(0);
        ptr = (UART0 + LCR) as *mut u8;
        ptr.write_volatile(LCR_BAUD_LATCH);
        ptr = (UART0 + 0) as *mut u8;
        ptr.write_volatile(0x03);
        ptr = (UART0 + 1) as *mut u8;
        ptr.write_volatile(0x00);
        ptr = (UART0 + LCR) as *mut u8;
        ptr.write_volatile(LCR_EIGHT_BITS);
        ptr = (UART0 + FCR) as *mut u8;
        ptr.write_volatile(FCR_FIFO_ENABLE | FCR_FIFO_CLEAR);
        ptr = (UART0 + IER) as *mut u8;
        ptr.write_volatile(IER_RX_ENABLE | IER_TX_ENABLE);
    }
    unsafe {
        let ptr = (UART0 + THR) as *mut u8;
        ptr.write_volatile(b'a');
    }
    loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{};
}