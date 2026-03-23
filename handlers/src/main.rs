#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(handlers::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use handlers::{println, serial_println};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    handlers::init();

    println!("Hello World!");

    x86_64::instructions::interrupts::int3();

    println!("It did not crash!");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    serial_println!("PANIC: {}", info);
    loop {}
}
