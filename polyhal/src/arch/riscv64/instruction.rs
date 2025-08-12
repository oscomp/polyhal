/// Riscv64 ebreak instruction.
#[inline]
pub fn ebreak() {
    unsafe {
        riscv::asm::ebreak();
    }
}

#[inline]
pub fn hlt() {
    unsafe {
        riscv::register::sstatus::clear_sie();
        riscv::asm::wfi();
        riscv::register::sstatus::set_sie();
    }
}

/// Call SBI_SHUTDOWN to close the machine. Exit qemu if you are using qemu.
#[inline]
pub fn shutdown() -> ! {
    // sbi_rt::legacy::shutdown();
    sbi_rt::system_reset(sbi_rt::Shutdown, sbi_rt::NoReason);
    unreachable!()
}
