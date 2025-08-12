#[cfg(target_arch = "aarch64")]
pub mod aarch64;
#[cfg(target_arch = "loongarch64")]
pub mod loongarch64;
#[cfg(target_arch = "riscv64")]
pub mod riscv64;
#[cfg(target_arch = "x86_64")]
pub mod x86_64;
pub_use_arch!(consts, hart_id);

pub const MEM_VECTOR_CAPACITY: usize = 0x20;
