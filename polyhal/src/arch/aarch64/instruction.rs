use crate::arch::aarch64::psci;

#[inline]
pub fn ebreak() {
    unsafe {
        core::arch::asm!("brk 0");
    }
}

/// Close the computer. Call PSCI.
#[inline]
pub fn shutdown() -> ! {
    psci::system_off()
}
