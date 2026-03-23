#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(handlers::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    handlers::init();
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    handlers::test_panic_handler(info)
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
