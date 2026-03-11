#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(integration::test_runner)]
#![reexport_test_harness_main = "test_main"]

use integration::{serial_println, qemu_quit};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    serial_println!("[Pass]");
    qemu_quit(0x10);
    loop {}
}

fn bad_assertion() {
    assert!(false);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    integration::test_runner(&[&bad_assertion]);
    qemu_quit(0x11);
    loop {}
}
