use core::cell::{RefCell, RefMut};
use arm_pl011::pl011::Pl011Uart;

pub const PSCI_0_2_FN_BASE: u32 = 0x84000000;
pub const PSCI_0_2_FN_SYSTEM_OFF: u32 = PSCI_0_2_FN_BASE + 8;

const UART_BASE: usize = axconfig::UART_PADDR + axconfig::PHYS_VIRT_OFFSET;

static UART: EarlyCon = EarlyCon::new();

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

/// Safety:
/// EarlyCon only can be used in early-stage of boot.
/// At that time, there's only one running thread.
/// When entering multi-task, disable earlycon and switch to formal console.
struct EarlyCon {
    inner: RefCell<Pl011Uart>,
}

impl EarlyCon {
    pub const fn new() -> Self {
        Self {
            inner: RefCell::new(Pl011Uart::new(UART_BASE as *mut u8)),
        }
    }

    pub fn get_mut(&self) -> RefMut<'_, Pl011Uart> {
        self.inner.borrow_mut()
    }
}

unsafe impl Sync for EarlyCon {}
