#![no_std]

mod platform;
mod time;

/// Write a slice of bytes to the console.
pub fn write_bytes(bytes: &[u8]) {
    for c in bytes {
        platform::putchar(*c);
    }
    sbi_rt::system_reset(sbi_rt::Shutdown, sbi_rt::NoReason);
}

pub fn time() -> core::time::Duration {
    time::current_time()
}

pub fn cpu_id() -> Option<usize> {
    None
}

pub fn task_id() -> Option<u64> {
    None
}
