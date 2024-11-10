Kernel early-stage console.

Use it before formal console can be activated during a kernel's
bootstrapping process.
Once formal console can be used, disable early console.

## Examples

```rust
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "Rust" fn runtime_main(_cpu_id: usize, _dtb_pa: usize) {
    let msg = "\n[rt_early_console]: ok!\n";
    early_console::write_bytes(msg.as_bytes());
    axhal::misc::terminate();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    arch_boot::panic(info)
}
```

## Functions

### write_bytes
```rust
fn write_bytes(bytes: &[u8]);
```
Write a slice of bytes to the console.

### read_bytes
```rust
fn read_bytes(bytes: &mut [u8]) -> usize;
```
Read a slice of bytes from the console.
