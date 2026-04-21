#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use clock::{println, serial_println};
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    clock::init();
    println!("Enter time as 6 digits (HHMMSS):");

    let mut clock_started = false;
    let mut last_secs = usize::MAX;

    loop {
        let running = clock::interrupts::CLOCK_RUNNING.load(core::sync::atomic::Ordering::Relaxed);

        if running {
            if !clock_started {
                clock_started = true;
                // clear screen by printing blank lines
                for _ in 0..25 {
                    println!("");
                }
            }

            let ticks = clock::interrupts::TICKS.load(core::sync::atomic::Ordering::Relaxed);
            let start = clock::interrupts::START_TICKS.load(core::sync::atomic::Ordering::Relaxed);
            let digits = *clock::interrupts::INPUT_DIGITS.lock();

            // PIT fires at ~18.2 Hz so divide by 18 for approximate seconds
            let elapsed_secs = (ticks - start) / 18;

            if elapsed_secs != last_secs {
                last_secs = elapsed_secs;

                let h = digits[0] as usize * 10 + digits[1] as usize;
                let m = digits[2] as usize * 10 + digits[3] as usize;
                let s = digits[4] as usize * 10 + digits[5] as usize;

                let total = h * 3600 + m * 60 + s + elapsed_secs;
                let hh = (total / 3600) % 24;
                let mm = (total % 3600) / 60;
                let ss = total % 60;

                // Print time on its own line, overwriting previous
                println!("{:02}:{:02}:{:02}", hh, mm, ss);
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
    clock::test_panic_handler(info)
}
