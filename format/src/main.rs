#![no_std]
#![no_main]

mod vga;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("PANIC: {}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {

    println!("Hello {}", "world");
    println!("Number: {}", 42);

    loop {}
}
