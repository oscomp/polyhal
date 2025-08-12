cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        pub use crate::arch::x86_64::instruction::{ebreak, shutdown};
    } else if #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))] {
        pub use crate::arch::riscv64::instruction::{ebreak, shutdown};
    } else if #[cfg(target_arch = "aarch64")] {
        pub use crate::arch::aarch64::instruction::{ebreak, shutdown};
    } else if #[cfg(target_arch = "loongarch64")] {
        pub use crate::arch::loongarch64::instruction::{ebreak, shutdown};
    }
}
