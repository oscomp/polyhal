pub mod barrier;
pub mod consts;
pub mod instruction;
pub mod irq;
pub mod kcontext;
pub mod multicore;
pub mod timer;

#[inline]
pub fn hart_id() -> usize {
    loongArch64::register::cpuid::read().core_id()
}
