
const PLIC: i32 = 0x0c00_0000;
const PLIC_PRIORITY: i32 = PLIC + 0x0;
const PLIC_PENDING: i32 = PLIC + 0x1000;
macro_rules! PLIC_SENABLE {
    ($hart:expr) => {
        PLIC + 0x2080 + ($hart)*0x100
    };
}
macro_rules! PLIC_SPRIORITY {
    ($hart:expr) => {
        PLIC + 0x201000 + (&hart)*0x2000
    };
}
macro_rules! PLIC_SCLAIM {
    ($hart:expr) => {
        PLIC + 0x201004 + ($hart)*0x2000
    };
}