#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use mapping::{println, serial_println};
use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    mapping::init();

    let offset = x86_64::VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { mapping::memory::init(offset) };

    // Part 1: EmptyFrameAllocator - map page 0 to VGA buffer
    let mut frame_allocator = mapping::memory::EmptyFrameAllocator;
    let page = x86_64::structures::paging::Page::containing_address(
        x86_64::VirtAddr::new(0)
    );
    mapping::memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // Write "New!" to screen through the new mapping
    let ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { ptr.write_volatile(0x_f021_f077_f065_f04e) };
    println!("Mapping with EmptyFrameAllocator worked!");

    // Part 2: try mapping 0xdeadbeaf000 with EmptyFrameAllocator - should fail
    // (comment out Part 1 mapping and uncomment this to test)
    // let page = x86_64::structures::paging::Page::containing_address(
    //     x86_64::VirtAddr::new(0xdeadbeaf000)
    // );
    // mapping::memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // Part 3: BootInfoFrameAllocator - map 0xdeadbeaf000 properly
    let mut frame_allocator = unsafe {
        mapping::memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    let page = x86_64::structures::paging::Page::containing_address(
        x86_64::VirtAddr::new(0xdeadbeaf000)
    );
    mapping::memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
    let ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { ptr.write_volatile(0x_f021_f077_f065_f04e) };
    println!("Mapping with BootInfoFrameAllocator worked!");

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
    mapping::test_panic_handler(info)
}
