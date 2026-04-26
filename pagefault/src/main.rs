#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use pagefault::{println, serial_println};
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    pagefault::init();

    println!(
        "Level 4 page table at: {:?}",
        x86_64::registers::control::Cr3::read().0.start_address()
    );

    let ptr = 0x20503b as *mut u8;

    // Test 1: read from a code page - should work
    unsafe { let x = *ptr; }
    println!("read worked");

    // Test 2: write to a code page - should page fault with PROTECTION_VIOLATION
    unsafe { *ptr = 42; }
    println!("write worked");

    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("PANIC: {}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    pagefault::test_panic_handler(info)
}
