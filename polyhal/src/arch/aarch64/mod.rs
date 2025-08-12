pub mod consts;
pub mod instruction;
pub mod irq;
pub mod kcontext;
pub mod multicore;
pub mod psci;
pub mod timer;

use aarch64_cpu::registers::{Readable, MPIDR_EL1};

#[inline]
pub fn hart_id() -> usize {
    MPIDR_EL1.read(MPIDR_EL1::Aff0) as _
}
