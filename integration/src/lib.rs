#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(integration::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[macro_use]
pub mod vga;
pub mod serial;

use core::panic::PanicInfo;

pub fn qemu_quit(exit_code: u32) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code);
    }
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    qemu_quit(0x10); // Success
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    qemu_quit(0x11); // Failure
    loop {}
}
