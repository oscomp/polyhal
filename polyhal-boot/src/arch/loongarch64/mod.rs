use core::arch::global_asm;
use loongArch64::register::euen;
use polyhal::arch::loongarch64::hart_id;
use polyhal::info::BOOT_INFO;
use polyhal::percpu::set_local_thread_pointer;
use polyhal::{
    consts::QEMU_DTB_ADDR,
    ctor::{ph_init_iter, CtorType},
    mem::parse_system_info,
};

global_asm!(
    include_str!("head.S"),
    entry = sym rust_tmp_main,
    secondary_entry = sym rust_secondary_main,
);

/// Rust temporary entry point
///
/// This function will be called after assembly boot stage.
pub fn rust_tmp_main(hart_id: usize) {
    let _ = BOOT_INFO.get_mut().parse_dtb(QEMU_DTB_ADDR);
    ph_init_iter(CtorType::Primary).for_each(|x| (x.func)());
    set_local_thread_pointer(hart_id);
    // Initialize CPU Configuration.
    init_cpu();
    ph_init_iter(CtorType::Cpu).for_each(|x| (x.func)());
    parse_system_info();
    ph_init_iter(CtorType::Platform).for_each(|x| (x.func)());
    ph_init_iter(CtorType::HALDriver).for_each(|x| (x.func)());

    super::call_real_main(hart_id);
}

/// Initialize CPU Configuration.
fn init_cpu() {
    // Enable floating point
    euen::set_fpe(true);

    // Initialzie Timer
    // timer::init_timer();
}

/// The entry point for the second core.
pub fn rust_secondary_main() {
    set_local_thread_pointer(hart_id());
    // Initialize CPU Configuration.
    init_cpu();

    super::call_real_main(hart_id());
}
