#![allow(static_mut_refs)]

use core::fmt;
use core::fmt::Write;
use spin::Mutex; // Corrected: Mutex needs a capital M
use lazy_static::lazy_static;

// VGA Constants
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

pub struct Writer {
    column_position: usize,
    row_position: usize,
    color_code: u8,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row_position;
                let col = self.column_position;
                let offset = (row * BUFFER_WIDTH + col) * 2;

                unsafe {
                    *VGA_BUFFER.add(offset) = byte;
                    *VGA_BUFFER.add(offset + 1) = self.color_code;
                }
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        if self.row_position < BUFFER_HEIGHT - 1 {
            self.row_position += 1;
        } else {
            for y in 1..BUFFER_HEIGHT {
                for x in 0..BUFFER_WIDTH {
                    unsafe {
                        let src_offset = (y * BUFFER_WIDTH + x) * 2;
                        let dst_offset = ((y - 1) * BUFFER_WIDTH + x) * 2;
                        *VGA_BUFFER.add(dst_offset) = *VGA_BUFFER.add(src_offset);
                        *VGA_BUFFER.add(dst_offset + 1) = *VGA_BUFFER.add(src_offset + 1);
                    }
                }
            }
            self.clear_row(BUFFER_HEIGHT - 1);
        }
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = b' ';
        for x in 0..BUFFER_WIDTH {
            unsafe {
                let offset = (row * BUFFER_WIDTH + x) * 2;
                *VGA_BUFFER.add(offset) = blank;
                *VGA_BUFFER.add(offset + 1) = self.color_code;
            }
        }
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


lazy_static! {
    /// Global interface for the VGA Writer
    /// Wrapped in a Mutex for thread-safety (critical for interrupts!)
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        row_position: 0,
        color_code: 0x0f, // White on black
    });
}

/// Internal print function used by macros
pub fn _print(args: fmt::Arguments) {
    use x86_64::instructions::interrupts;

    // This is the most important line for stability
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

// MACROS
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
