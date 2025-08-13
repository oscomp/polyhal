use core::hint::spin_loop;

use crate::utils::MutexNoIrq;
use ns16550a::Uart;

use super::DebugConsole;

#[cfg(not(board = "2k1000"))]
const UART_ADDR: usize = 0x01FE001E0 | crate::arch::consts::VIRT_ADDR_START;
#[cfg(board = "2k1000")]
const UART_ADDR: usize = 0x800000001fe20000;
// 0x800000001fe20000ULL
static COM1: MutexNoIrq<Uart> = MutexNoIrq::new(Uart::new(UART_ADDR));

impl DebugConsole {
    /// Writes a byte to the console.
    #[inline]
    pub fn putchar(ch: u8) {
        let com = COM1.lock();
        if ch == b'\n' {
            while com.put(b'\r').is_none() {
                spin_loop();
            }
        }
        while com.put(ch).is_none() {
            spin_loop();
        }
    }

    /// read a byte, return -1 if nothing exists.
    #[inline]
    pub fn getchar() -> Option<u8> {
        COM1.lock().get()
    }
}
