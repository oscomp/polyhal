pub mod consts;
pub mod irq;
pub mod timer;

#[inline]
pub fn hart_id() -> usize {
    loongArch64::register::cpuid::read().core_id()
}
