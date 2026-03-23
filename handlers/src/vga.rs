#![allow(static_mut_refs)]

use core::fmt;
use core::fmt::Write;

// VGA Constants
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

pub struct Writer {
    column_position: usize,
    row_position: usize, // Added to track the current line for scrolling
    color_code: u8,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                // If we reach the end of the line, wrap automatically
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row_position;
                let col = self.column_position;

                // Calculate memory offset: (row * width + col) * 2 bytes per cell
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
            // Just move to the next line if there's space
            self.row_position += 1;
        } else {
            // SCROLLING: Move all rows up by one
            for y in 1..BUFFER_HEIGHT {
                for x in 0..BUFFER_WIDTH {
                    unsafe {
                        let src_offset = (y * BUFFER_WIDTH + x) * 2;
                        let dst_offset = ((y - 1) * BUFFER_WIDTH + x) * 2;
                        // Move character
                        *VGA_BUFFER.add(dst_offset) = *VGA_BUFFER.add(src_offset);
                        // Move color (Requirement: "Verify coloration is maintained")
                        *VGA_BUFFER.add(dst_offset + 1) = *VGA_BUFFER.add(src_offset + 1);
                    }
                }
            }
            // Clear the now-empty bottom row
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

// Global Writer instance
static mut WRITER: Writer = Writer {
    column_position: 0,
    row_position: 0,
    color_code: 0x0f, // White text on Black background
};

/// Internal print function used by macros
pub fn _print(args: fmt::Arguments) {
    unsafe {
        WRITER.write_fmt(args).unwrap();
    }
}

// MACROS: These must use $crate so they work in both main.rs and tests/
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
