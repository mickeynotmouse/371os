#![allow(static_mut_refs)]

use core::fmt;
use core::fmt::Write;

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const BUFFER_WIDTH: usize = 80;

pub struct Writer {
    column_position: usize,
    color_code: u8,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => unsafe {
                let offset = self.column_position * 2;

                *VGA_BUFFER.add(offset) = byte;
                *VGA_BUFFER.add(offset + 1) = self.color_code;

                self.column_position += 1;

                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
            },
        }
    }

    fn new_line(&mut self) {
        self.column_position = 0;
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

static mut WRITER: Writer = Writer {
    column_position: 0,
    color_code: 0x0f, // white on black
};

pub fn _print(args: fmt::Arguments) {
    unsafe {
        WRITER.write_fmt(args).unwrap();
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::vga::_print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::vga::_print(format_args!("{}\n", format_args!($($arg)*)));
    });
}
