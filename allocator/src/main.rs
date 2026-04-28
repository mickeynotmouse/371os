#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(allocator::_test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use allocator::{println, serial_println};
use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    allocator::init();

    let offset = x86_64::VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { allocator::memory::init(offset) };
    let mut frame_allocator = unsafe {
        allocator::memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    println!("Hello world{}","!");

    let b = alloc::boxed::Box::new(371);
    println!("Hello box containing {}!", *b);
    println!("Hello box at {:p}!", b);

    loop {
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
    allocator::test_panic_handler(info)
}
