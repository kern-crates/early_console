use crate::earlydev::EarlyDev;

pub const VIRT_ADDR_START: usize = 0x9000_0000_0000_0000;
const UART_ADDR: usize = 0x01FE001E0 | VIRT_ADDR_START;
static COM1: EarlyDev<Uart> = EarlyDev::new(Uart::new(UART_ADDR));

pub fn console_init() {
}

/// Writes a byte to the console.
pub fn putchar(c: u8) {
    if c == b'\n' {
        COM1.get_mut().putchar(b'\r');
    }
    COM1.get_mut().putchar(c)
}

pub fn terminate() -> ! {
    loop {}
}

pub struct Uart {
    base_address: usize,
}

impl Uart {
    pub const fn new(base_address: usize) -> Self {
        Uart { base_address }
    }

    pub fn putchar(&mut self, c: u8) {
        let ptr = self.base_address as *mut u8;
        loop {
            unsafe {
                if ptr.add(5).read_volatile() & (1 << 5) != 0 {
                    break;
                }
            }
        }
        unsafe {
            ptr.add(0).write_volatile(c);
        }
    }
}
