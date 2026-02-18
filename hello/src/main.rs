#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    unsafe {
        // Your "payload" integers
        let ints: [i32; 3] = [1819043144, 1870078063, 560229490];

        // Cast [i32; 3] -> [u8; 12] -> &str
        let bytes: &[u8; 12] = &core::mem::transmute(ints);
        let s: &str = core::str::from_utf8_unchecked(bytes);

        // Write string to VGA buffer at top-left
        let vga_buffer = 0xb8000 as *mut u8;
        for (i, &b) in s.as_bytes().iter().enumerate() {
            vga_buffer.add(i * 2).write_volatile(b);      // ASCII byte
            vga_buffer.add(i * 2 + 1).write_volatile(0x0f); // color byte (white on black)
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

