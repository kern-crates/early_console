use arm_pl011::pl011::Pl011Uart;
use crate::earlydev::EarlyDev;

pub const PSCI_0_2_FN_BASE: u32 = 0x84000000;
pub const PSCI_0_2_FN_SYSTEM_OFF: u32 = PSCI_0_2_FN_BASE + 8;

const UART_BASE: usize = axconfig::UART_PADDR + axconfig::PHYS_VIRT_OFFSET;

static UART: EarlyDev<Pl011Uart> =
    EarlyDev::new(Pl011Uart::new(UART_BASE as *mut u8));

pub fn console_init() {
    UART.get_mut().init();
}

/// Writes a byte to the console.
pub fn putchar(c: u8) {
    let mut uart = UART.get_mut();
    match c {
        b'\n' => {
            uart.putchar(b'\r');
            uart.putchar(b'\n');
        }
        c => uart.putchar(c),
    }
}

pub fn terminate() -> ! {
    unsafe {
        core::arch::asm!(
            "hvc #0",
            in("x0") PSCI_0_2_FN_SYSTEM_OFF,
        )
    }
    loop {}
}
