//! Timer module.
//!
//!

use core::time::Duration;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        pub use crate::arch::x86_64::timer::{get_ticks, get_freq, set_next_timer};
    } else if #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))] {
        pub use crate::arch::riscv64::timer::{get_ticks, get_freq, set_next_timer};
    } else if #[cfg(target_arch = "aarch64")] {
        pub use crate::arch::aarch64::timer::{get_ticks, get_freq, set_next_timer};
    } else if #[cfg(target_arch = "loongarch64")] {
        pub use crate::arch::loongarch64::timer::{get_ticks, get_freq, set_next_timer};
    }
}

/// Get current time
///
/// # Return
///
/// Return [Duration] with current time
#[inline]
pub fn current_time() -> Duration {
    let ticks = get_ticks();
    let freq = get_freq();
    Duration::new(ticks / freq, ((ticks % freq) * 1_000_000_000 / freq) as u32)
}
