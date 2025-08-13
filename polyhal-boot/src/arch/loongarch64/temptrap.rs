use core::arch::global_asm;

use loongArch64::register::{
    badv, ecfg, eentry, era,
    estat::{self, Exception, Trap},
};

use crate::arch::loongarch64::unaligned::emulate_load_store_insn;

global_asm!(include_str!("temptrap.S"));

#[no_mangle]
pub extern "C" fn handle_exception(regs: &mut [usize; 32]) {
    let estat = estat::read();
    match estat.cause() {
        Trap::Exception(Exception::AddressNotAligned) => unsafe { emulate_load_store_insn(regs) },
        _ => {
            panic!(
                "Unhandled trap {:?} @ {:#x} BADV: {:#x}",
                estat.cause(),
                era::read().raw(),
                badv::read().vaddr(),
            );
        }
    };
}

pub fn init() {
    extern "C" {
        fn _temptrap_entry();
    }
    ecfg::set_vs(0);
    eentry::set_eentry(_temptrap_entry as usize);
}
