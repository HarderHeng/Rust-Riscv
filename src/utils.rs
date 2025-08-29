
#[macro_export]
macro_rules! RegReadU8 {
    ($reg:expr) => {
        unsafe {
            let ptr = ($reg) as *mut u8;
            ptr.read_volatile()
        }
    };
}

#[macro_export]
macro_rules! RegWriteU8 {
    ($reg:expr, $val:expr) => {
        unsafe {
            let ptr = ($reg) as *mut u8;
            ptr.write_volatile($val as u8);
        }
    };
}

#[macro_export]
macro_rules! RegReadU16 {
    ($reg:expr) => {
        unsafe {
            let ptr = ($reg) as *mut u16;
            ptr.read_volatile()
        }
    };
}

#[macro_export]
macro_rules! RegWriteU16 {
    ($reg:expr, $val:expr) => {
        unsafe {
            let ptr = ($reg) as *mut u16;
            ptr.write_volatile($val as u16);
        }
    };
}

#[macro_export]
macro_rules! RegReadU32 {
    ($reg:expr) => {
        unsafe {
            let ptr = ($reg) as *mut u32;
            ptr.read_volatile()
        }
    };
}

#[macro_export]
macro_rules! RegWriteU32 {
    ($reg:expr, $val:expr) => {
        unsafe {
            let ptr = ($reg) as *mut u32;
            ptr.write_volatile($val as u32);
        }
    };
}
