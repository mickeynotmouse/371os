#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(integration::test_runner)]
#![reexport_test_harness_main = "test_main"]

use integration::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    integration::test_panic_handler(info)
}

#[test_case]
fn test_println_output() {
    println!("test_println_output...");
}

#[test_case]
fn test_println_many() {
    for i in 0..100 {
        println!("test_println_many line {}", i);
    }
}
