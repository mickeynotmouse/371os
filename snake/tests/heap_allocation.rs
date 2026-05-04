#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(allocator::_test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

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

    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    allocator::test_panic_handler(info)
}

#[test_case]
fn simple_allocation() {
    let heap_value_1 = alloc::boxed::Box::new(41);
    let heap_value_2 = alloc::boxed::Box::new(13);
    assert_eq!(*heap_value_1, 41);
    assert_eq!(*heap_value_2, 13);
}

#[test_case]
fn large_vec() {
    let n = 1000u64;
    let mut vec = alloc::vec::Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

#[test_case]
fn many_boxes() {
    for i in 0..allocator::allocator::HEAP_SIZE {
        let x = alloc::boxed::Box::new(i);
        assert_eq!(*x, i);
    }
}
