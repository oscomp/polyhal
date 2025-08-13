use loongArch64::ipi::{csr_mail_send, send_ipi_single};

use crate::consts::VIRT_ADDR_START;

// TODO: Boot a core with top pointer of the stack
pub fn boot_core(hart_id: usize, addr: usize, sp_top: usize) {
    csr_mail_send((addr | VIRT_ADDR_START) as _, hart_id, 0);
    csr_mail_send((sp_top | VIRT_ADDR_START) as _, hart_id, 1);
    send_ipi_single(hart_id, 1);
}
