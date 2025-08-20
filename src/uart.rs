macro_rules! _RegRead {
    ($reg:expr) => {
        unsafe {
            let ptr = ($reg) as *mut u8;
            ptr.read_volatile()
        }
    };
}
macro_rules! RegWrite {
    ($reg:expr, $val:expr) => {
        unsafe {
            let ptr = ($reg) as *mut u8;
            ptr.write_volatile($val)
        }
    };
}
const UART0:i32             =   0x10000000;
const THR:i32               =   0;
const IER:i32               =   1;
const _ISR:i32              =   2;
const LCR:i32               =   3;
const FCR:i32               =   2;
const LCR_BAUD_LATCH:u8     =   1<<7;
const LCR_EIGHT_BITS:u8     =   3<<0;
const FCR_FIFO_ENABLE:u8    =   1<<0;
const FCR_FIFO_CLEAR:u8     =   3<<1;
const IER_RX_ENABLE:u8      =   1<<0;
const IER_TX_ENABLE:u8      =   1<<1;

pub fn uart_init() {
    RegWrite!(UART0 + IER, 0);
    RegWrite!(UART0 + LCR, LCR_BAUD_LATCH);
    RegWrite!(UART0 + 0, 0x03);
    RegWrite!(UART0 + 1, 0x00);
    RegWrite!(UART0 + LCR, LCR_EIGHT_BITS);
    RegWrite!(UART0 + FCR, FCR_FIFO_ENABLE | FCR_FIFO_CLEAR);
    RegWrite!(UART0 + IER, IER_RX_ENABLE | IER_TX_ENABLE);
}

pub fn uart_print(str: &str) {
    for c in str.chars() {
        RegWrite!(UART0 + THR, c as u8);
    }
    RegWrite!(UART0 + THR, b'\n');
}