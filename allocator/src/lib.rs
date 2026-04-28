#![no_std]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::_test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

#[macro_use]
pub mod vga;
pub mod serial;
pub mod interrupts;
pub mod gdt;
pub mod memory;
pub mod allocator;

use core::panic::PanicInfo;

pub const QEMU_PASS: u32 = 0x10;
pub const QEMU_FAIL: u32 = 0x11;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    unsafe { interrupts::PICS.lock().write_masks(0xFC, 0xFF) };
    x86_64::instructions::interrupts::enable();
}

pub fn halt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn qemu_quit(exit_code: u32) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code);
    }
}

pub fn _test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for (i, test) in tests.iter().enumerate() {
        serial_print!("Initiating test 0x{:02x}...", i);
        test();
        serial_println!(" [Pass]");
    }
    qemu_quit(QEMU_PASS);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!(" [failed]");
    serial_println!("Error: {}", info);
    qemu_quit(QEMU_FAIL);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
