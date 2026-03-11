#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(integration::test_runner)] 
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use integration::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("OS Booting...");

    for i in 0..30 {
        println!("Line number: {}", i);
    }

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("KERNEL PANIC: {}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    integration::test_panic_handler(info)
}
