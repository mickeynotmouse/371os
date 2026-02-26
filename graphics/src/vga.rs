#[derive(Clone, Copy)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

impl Color {
    // convert u8 to Color
    pub fn from(value: u8) -> Self {
        match value % 16 {
            0 => Color::Black,
            1 => Color::Blue,
            2 => Color::Green,
            3 => Color::Cyan,
            4 => Color::Red,
            5 => Color::Magenta,
            6 => Color::Brown,
            7 => Color::LightGray,
            8 => Color::DarkGray,
            9 => Color::LightBlue,
            10 => Color::LightGreen,
            11 => Color::LightCyan,
            12 => Color::LightRed,
            13 => Color::Pink,
            14 => Color::Yellow,
            15 => Color::White,
            _ => Color::Black,
        }
    }
}

pub const BUFFER_WIDTH: usize = 80;
pub const BUFFER_HEIGHT: usize = 25;

// VGA Writer struct to draw background colors
pub struct Writer {
    buffer: *mut u8,
}

impl Writer {
    pub fn new() -> Self {
        Writer {
            buffer: 0xb8000 as *mut u8, // VGA memory
        }
    }

    // set background color of a cell (keeps char as space)
    pub fn set_bg_color(&mut self, x: usize, y: usize, color: Color) {
        if x >= BUFFER_WIDTH || y >= BUFFER_HEIGHT {
            return;
        }

        let offset = (y * BUFFER_WIDTH + x) * 2;
        unsafe {
            // ASCII space
            *self.buffer.add(offset) = b' ';
            // color byte: background << 4, foreground = 0
            *self.buffer.add(offset + 1) = (color as u8) << 4;
        }
    }
}
