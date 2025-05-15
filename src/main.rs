// main.rs
#![no_std]      // Stops linking to Rust standard library
#![no_main]     // Disables all rust-level entry points

use core::panic::PanicInfo;

// Static byte string
static HELLO: &[u8] = b"Hello World";

// _start function to overwrite system entry point
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;  // 0xb is light cyan 
        }
    }


    loop {}
}

// Function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

