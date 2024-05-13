pub fn console_init() {
}

/// Writes a byte to the console.
pub fn putchar(c: u8) {
    print!("{}", c as char);
}

pub fn terminate() -> ! {
    std::process::exit(0)
}
