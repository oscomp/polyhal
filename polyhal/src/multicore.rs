cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        pub use crate::arch::x86_64::multicore::boot_core;
    } else if #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))] {
        pub use crate::arch::riscv64::multicore::boot_core;
    } else if #[cfg(target_arch = "aarch64")] {
        pub use crate::arch::aarch64::multicore::boot_core;
    } else if #[cfg(target_arch = "loongarch64")] {
        pub use crate::arch::loongarch64::multicore::boot_core;
    }
}
