use crate::*;

const UART0: i32            =   0x10000000;
const THR: i32              =   0;
const IER: i32              =   1;
const _ISR: i32             =   2;
const LCR: i32              =   3;
const FCR: i32              =   2;
const LCR_BAUD_LATCH: i32   =   1<<7;
const LCR_EIGHT_BITS: i32   =   3<<0;
const FCR_FIFO_ENABLE: i32  =   1<<0;
const FCR_FIFO_CLEAR: i32   =   3<<1;
const IER_RX_ENABLE: i32    =   1<<0;
const IER_TX_ENABLE: i32    =   1<<1;

pub fn uart_init() {
    uart_0_init();
}

pub fn uart_print_str(str: &str) {
    uart_0_print(str);
}

fn uart_0_init() {
    let uartbaseaddr = UART0;
    RegWriteU8!(uartbaseaddr + IER, 0);
    RegWriteU8!(uartbaseaddr + LCR, LCR_BAUD_LATCH);
    RegWriteU8!(uartbaseaddr + 0, 0x03);
    RegWriteU8!(uartbaseaddr + 1, 0x00);
    RegWriteU8!(uartbaseaddr + LCR, LCR_EIGHT_BITS);
    RegWriteU8!(uartbaseaddr + FCR, FCR_FIFO_ENABLE | FCR_FIFO_CLEAR);
    RegWriteU8!(uartbaseaddr + IER, IER_RX_ENABLE | IER_TX_ENABLE);
}

fn uart_0_print(str: &str) {
    let uartbaseaddr = UART0;
    for c in str.chars() {
        RegWriteU8!(uartbaseaddr + THR, c);
    }
    RegWriteU8!(uartbaseaddr + THR, b'\n');
}
fn uart_print_digit()