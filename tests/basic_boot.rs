// basic_boot.rs
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rain_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rain_os::println;

// Entry Point Function for integration testing
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();

    loop{}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rain_os::test_panic_handler(info)
}

// Test that println works

#[test_case]
fn test_println() {
    println!("test_println output");
}
