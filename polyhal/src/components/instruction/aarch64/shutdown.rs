use crate::arch::aarch64::psci;

/// Close the computer. Call PSCI.
#[inline]
pub fn shutdown() -> ! {
    psci::system_off()
}
