#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga;
mod colors;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    colors::image();
    loop {}
}
