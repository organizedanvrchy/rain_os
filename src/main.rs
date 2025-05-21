// main.rs

#![no_std] // Stops linking to Rust standard library
#![no_main] // Disables all rust-level entry points 
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"] // Set name of test framework function to test_main

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
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// Rust module to handle printing
mod vga_buffer;

// Serial module to handle print to console
mod serial;

// QEMU exit function with specified exit status 
// (different from default QEMU codes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

// Create and write to new port
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

// **Custom test framework**

// Test traits
pub trait Testable {
    fn run(&self) -> ();
}

// Implement trait for all types of T that Implement Fn() trait
impl<T> Testable for T
where
    T:Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

// Test Runner
#[cfg(test)]
// Slice of trait object (&[&dyn Testable]) that 
// references the Testable trait
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());     
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

// Panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop{}
}

// Test Case
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

