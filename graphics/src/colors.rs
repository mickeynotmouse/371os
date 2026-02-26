#![allow(dead_code)]

mod img;
use crate::vga::{Writer, Color};
use crate::colors::img::DATA;

pub fn colors() {
    let mut writer = Writer::new();

    for row in 0..crate::vga::BUFFER_HEIGHT {
        for col in 0..crate::vga::BUFFER_WIDTH {
            // Map column to one of the 16 VGA colors
            let color = (col * 16 / crate::vga::BUFFER_WIDTH) as u8;
            writer.set_bg_color(col, row, Color::from(color));
        }
    }
}

pub fn image() {
    let mut writer = Writer::new();

    let max_rows = core::cmp::min(DATA.len(), 25);

    for row in 0..max_rows {
        let line = &DATA[row];
        let max_cols = core::cmp::min(line.len(), 80);

        for col in 0..max_cols {
            let color = line[col];
            writer.set_bg_color(col, row, Color::from(color));
        }
    }
}
