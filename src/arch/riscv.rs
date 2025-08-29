#[macro_export]
macro_rules! r_mhartid {
    () => {{
        let hartid: usize;
        unsafe {
            core::arch::asm!(
                "csrr {}, mhartid",
                out(reg) hartid,
                options(nomem, nostack, preserves_flags)
            );
        }
        hartid
    }};
}