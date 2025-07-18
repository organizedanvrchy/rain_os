// src/main.rs

#![no_std] // Stops linking to Rust standard library
#![no_main] // Disables all rust-level entry points 
#![feature(custom_test_frameworks)]
#![test_runner(rain_os::test_runner)]
#![reexport_test_harness_main = "test_main"] // Set name of test framework function to test_main

use core::panic::PanicInfo;
use rain_os::println;

// _start function to overwrite system entry point
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    rain_os::init();

    //// Trigger stack overflow
    // fn stack_overflow() {
    //     stack_overflow();
    // }

    // stack_overflow();
    
    // Trigger page fault
    // let ptr = 0xdeadbeaf as *mut u8;
    // unsafe { *ptr = 42; }

    // Trigger page fault #2
    // let ptr = 0x2031b2 as *mut u8;

    // read from a code page
    // unsafe { let x = *ptr; }
    // println!("read worked!");

    // write to a code page
    // unsafe { *ptr = 42; }
    // println!("write worked!");
    
    // Accessing Page Tables
    // use x86_64::registers::control::Cr3;

    // let (level_4_page_table, _) = Cr3::read();
    // println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    #[cfg(test)]
    test_main();
    
    println!("It did not crash!");
    rain_os::hlt_loop();
}

// Function called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rain_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rain_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
