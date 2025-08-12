/// Kernel Context Arg Type.
///
/// Using this by Index and IndexMut trait bound on KContext.
#[derive(Debug)]
pub enum KContextArgs {
    /// Kernel Stack Pointer
    KSP,
    /// Kernel Thread Pointer
    KTP,
    /// Kernel Program Counter
    KPC,
}

cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        pub use crate::arch::x86_64::kcontext::{context_switch, context_switch_pt};
    } else if #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))] {
        pub use crate::arch::riscv64::kcontext::{context_switch, context_switch_pt};
    } else if #[cfg(target_arch = "aarch64")] {
        pub use crate::arch::aarch64::kcontext::{context_switch, context_switch_pt};
    } else if #[cfg(target_arch = "loongarch64")] {
        pub use crate::arch::loongarch64::kcontext::{context_switch, context_switch_pt};
        #[cfg(feature = "fp_simd")]
        pub use crate::arch::loongarch64::kcontext::{save_fp_regs, restore_fp_regs};
    }
}
