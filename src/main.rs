// main.rs

// Stops linking to Rust standard library
#![no_std]
// Disables all rust-level entry points
#![no_main]     
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
// Set name of test framework function to test_main
#![reexport_test_harness_main = "test_main"] 

use core::panic::PanicInfo;

// _start function to overwrite system entry point
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

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

// Custom test framework 
#[cfg(test)]
// Slice of trait object (&[&dyn Fn()]) that references the Fn() trait
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());     
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

