#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(exceptions::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use exceptions::{println, serial_println};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    exceptions::init();

    println!("VGA 2: OS Ready");

    x86_64::instructions::interrupts::int3();

    println!("VGA 3: SUCCESS!");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    serial_println!("PANIC: {}", info);
    loop {}
}
