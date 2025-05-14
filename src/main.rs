// main.rs
#![no_std]
#![no_main]

use core::panic::PanicInfo;

// _start function to overwrite system entry point
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}

// Function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

