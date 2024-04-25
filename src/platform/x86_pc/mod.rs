pub mod time;
pub mod uart16550;
pub use uart16550::{console_init, putchar};
use x86_64::instructions::port::PortWriteOnly;

pub fn terminate() -> ! {
    #[cfg(platform = "x86_64-pc-oslab")]
    {
        unsafe { PortWriteOnly::new(0x64).write(0xfeu8) };
    }

    #[cfg(platform = "x86_64-qemu-q35")]
    unsafe {
        PortWriteOnly::new(0x604).write(0x2000u16)
    };

    loop {}
}
