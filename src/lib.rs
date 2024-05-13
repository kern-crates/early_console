#![cfg_attr(not(target_os = "linux"), no_std)]
#![feature(exclusive_wrapper)]

#[allow(dead_code)]
mod earlydev;
mod platform;

pub fn init() {
    platform::console_init();
}

/// Write a slice of bytes to the console.
pub fn write_bytes(bytes: &[u8]) {
    for c in bytes {
        platform::putchar(*c);
    }
    platform::terminate();
}
