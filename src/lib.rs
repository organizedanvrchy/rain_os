// src/lib.rs

#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

// Module to handle print to console
pub mod serial;
// Module to handle print to VGA buffer
pub mod vga_buffer;
// Interrupt handler module
pub mod interrupts;
// Task State Segment module
pub mod gdt;

// Load IDT and GDT, and PIC
pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe {interrupts::PICS.lock().initialize()};
    x86_64::instructions::interrupts::enable();
}

// hlt loop to halt CPU until next interrupt
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
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

// Slice of trait object (&[&dyn Testable]) that 
// references the Testable trait
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());     
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop{}
}

// **QEMU exit function** with specified exit status 
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

// **Entry Point for `cargo test`**
#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    loop{}
}

// Panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

// Breakpoint Exception Test
#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
