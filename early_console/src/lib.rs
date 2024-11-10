//! Kernel early-stage console.
//!
//! Use it before formal console can be activated during a kernel's
//! bootstrapping process.
//! Once formal console can be used, disable early console.

#![no_std]

mod platform;
mod time;

pub use platform::{getchar, putchar};

/// Init early console.
pub fn init() {
    #[cfg(target_arch = "x86_64")]
    platform::console_init();
}

/// Write a slice of bytes to the console.
pub fn write_bytes(bytes: &[u8]) {
    for c in bytes {
        platform::putchar(*c);
    }
}

/// Read a slice of bytes from the console.
pub fn read_bytes(bytes: &mut [u8]) -> usize {
    let mut read_len = 0;
    while read_len < bytes.len() {
        if let Some(c) = platform::getchar() {
            bytes[read_len] = c;
            read_len += 1;
        } else {
            break;
        }
    }
    read_len
}

/// Get current time.
pub fn time() -> core::time::Duration {
    time::current_time()
}

/// Get current cpu ID.
pub fn cpu_id() -> Option<usize> {
    None
}

/// Get current task ID.
pub fn task_id() -> Option<u64> {
    None
}
