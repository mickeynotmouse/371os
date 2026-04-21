#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use osirs::{println, serial_println};
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    osirs::init();

    println!("Ready for input:");

    loop {
        while let Some(scancode) = osirs::interrupts::pop_scancode() {
            unsafe {
                use pc_keyboard::DecodedKey;
                if let Ok(Some(key_event)) =
                    (*core::ptr::addr_of_mut!(osirs::interrupts::KB_RAW)).add_byte(scancode)
                {
                    if let Some(key) =
                        (*core::ptr::addr_of_mut!(osirs::interrupts::KB_RAW))
                            .process_keyevent(key_event)
                    {
                        match key {
                            DecodedKey::Unicode(c) => osirs::print!("{}", c),
                            DecodedKey::RawKey(k) => osirs::print!("{:?}", k),
                        }
                    }
                }
            }
        }
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
    osirs::test_panic_handler(info)
}
