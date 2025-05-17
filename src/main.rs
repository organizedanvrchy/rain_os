// main.rs
#![no_std]      // Stops linking to Rust standard library
#![no_main]     // Disables all rust-level entry points

use core::panic::PanicInfo;

// _start function to overwrite system entry point
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    loop {}
}

// Function called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Rust module to handle printing
mod vga_buffer;
