use alloc::collections::VecDeque;
use spin::Mutex;
use core::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const WIDTH: usize = 80;
const HEIGHT: usize = 25;

// Box drawing characters (CP437)
const H_BAR: u8 = 0xC4;
const V_BAR: u8 = 0xB3;
const TL:    u8 = 0xDA;
const TR:    u8 = 0xBF;
const BL:    u8 = 0xC0;
const BR:    u8 = 0xD9;

pub const FOOD:  u8 = 0xA2;
pub const SNAKE: u8 = 0xDB;
const EMPTY: u8 = b' ';

const COLOR_NORMAL: u8 = 0x0F;
const COLOR_SNAKE:  u8 = 0x0A; // green
const COLOR_FOOD:   u8 = 0x0C; // red
const COLOR_BORDER: u8 = 0x0F;

// Direction constants
pub const DIR_RIGHT: u8 = 0;
pub const DIR_LEFT:  u8 = 1;
pub const DIR_UP:    u8 = 2;
pub const DIR_DOWN:  u8 = 3;

// Game state
pub static DIRECTION: AtomicU8 = AtomicU8::new(DIR_RIGHT);
pub static FOOD_ROW: AtomicUsize = AtomicUsize::new(12);
pub static FOOD_COL: AtomicUsize = AtomicUsize::new(20);
pub static RAND_SEED: AtomicUsize = AtomicUsize::new(1);

pub static SNEK: Mutex<VecDeque<[usize; 2]>> = Mutex::new(VecDeque::new());

pub fn write_char(row: usize, col: usize, ch: u8, color: u8) {
    let offset = (row * WIDTH + col) * 2;
    unsafe {
        *VGA_BUFFER.add(offset) = ch;
        *VGA_BUFFER.add(offset + 1) = color;
    }
}

pub fn read_char(row: usize, col: usize) -> u8 {
    let offset = (row * WIDTH + col) * 2;
    unsafe { *VGA_BUFFER.add(offset) }
}

pub fn clear_screen() {
    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            write_char(row, col, EMPTY, COLOR_NORMAL);
        }
    }
}

pub fn draw_border() {
    for col in 0..WIDTH {
        write_char(0, col, H_BAR, COLOR_BORDER);
        write_char(HEIGHT - 1, col, H_BAR, COLOR_BORDER);
    }
    for row in 0..HEIGHT {
        write_char(row, 0, V_BAR, COLOR_BORDER);
        write_char(row, WIDTH - 1, V_BAR, COLOR_BORDER);
    }
    write_char(0, 0, TL, COLOR_BORDER);
    write_char(0, WIDTH - 1, TR, COLOR_BORDER);
    write_char(HEIGHT - 1, 0, BL, COLOR_BORDER);
    write_char(HEIGHT - 1, WIDTH - 1, BR, COLOR_BORDER);
}

pub fn next_food_pos() -> (usize, usize) {
    // Simple pseudo-random using seed
    let seed = RAND_SEED.load(Ordering::Relaxed);
    let new_seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    RAND_SEED.store(new_seed, Ordering::Relaxed);
    let row = (new_seed >> 33) % (HEIGHT - 2) + 1;
    let col = (new_seed >> 17) % (WIDTH - 2) + 1;
    // Make sure it doesn't overlap snake
    let snek = SNEK.lock();
    for segment in snek.iter() {
        if segment[0] == row && segment[1] == col {
            drop(snek);
            return next_food_pos(); // try again
        }
    }
    (row, col)
}

pub fn place_food(row: usize, col: usize) {
    FOOD_ROW.store(row, Ordering::Relaxed);
    FOOD_COL.store(col, Ordering::Relaxed);
    write_char(row, col, FOOD, COLOR_FOOD);
}

pub fn init_game() {
    clear_screen();
    draw_border();

    // Initialize snake with 3 segments in the middle
    let mut snek = SNEK.lock();
    snek.push_back([12, 10]);
    snek.push_back([12, 11]);
    snek.push_back([12, 12]);
    drop(snek);

    // Draw initial snake
    write_char(12, 10, SNAKE, COLOR_SNAKE);
    write_char(12, 11, SNAKE, COLOR_SNAKE);
    write_char(12, 12, SNAKE, COLOR_SNAKE);

    // Place initial food at hardcoded position
    place_food(5, 20);

    // Press any key to start message
    let msg = b"PRESS ANY WASD-KEY TO START";
    let col = (WIDTH - msg.len()) / 2;
    for (i, &ch) in msg.iter().enumerate() {
        write_char(23, col + i, ch, 0x0B); // cyan on black
    }
}

pub fn update() {
    let dir = DIRECTION.load(Ordering::Relaxed);
    let mut snek = SNEK.lock();

    // Get head position
    let head = *snek.back().unwrap();
    let (new_row, new_col) = match dir {
        DIR_RIGHT => (head[0], head[1] + 1),
        DIR_LEFT  => (head[0], head[1] - 1),
        DIR_UP    => (head[0] - 1, head[1]),
        DIR_DOWN  => (head[0] + 1, head[1]),
        _         => (head[0], head[1] + 1),
    };

    // Check what's at new position
    let cell = read_char(new_row, new_col);

    if cell == H_BAR || cell == V_BAR || cell == TL ||
       cell == TR || cell == BL || cell == BR || cell == SNAKE {
        // Hit wall or self - game over
        drop(snek);
        game_over();
        return;
    }

    let ate_food = cell == FOOD;

    // Add new head
    snek.push_back([new_row, new_col]);
    write_char(new_row, new_col, SNAKE, COLOR_SNAKE);

    if ate_food {
        // Don't remove tail - snake grows
        // Update seed with current tick for randomness
        let tick = crate::interrupts::TICKS.load(Ordering::Relaxed);
        RAND_SEED.store(tick.wrapping_mul(2654435761), Ordering::Relaxed);

        // Check victory - snake fills entire play area
        let play_area = (WIDTH - 2) * (HEIGHT - 2);
        if snek.len() >= play_area {
            drop(snek);
            game_win();
            return;
        }

        drop(snek);
        let (fr, fc) = next_food_pos();
        place_food(fr, fc);
    } else {
        // Remove tail
        let tail = snek.pop_front().unwrap();
        write_char(tail[0], tail[1], EMPTY, COLOR_NORMAL);
        drop(snek);
    }
}

pub fn game_over() {
    clear_screen();
    // Print GAME OVER in the middle
    let msg = b"GAME OVER";
    let col = (WIDTH - msg.len()) / 2;
    for (i, &ch) in msg.iter().enumerate() {
        write_char(12, col + i, ch, 0x4F); // white on red
    }
    crate::qemu_quit(crate::QEMU_FAIL);
}

pub fn game_win() {
    clear_screen();
    let msg = b"YOU WIN!";
    let col = (WIDTH - msg.len()) / 2;
    for (i, &ch) in msg.iter().enumerate() {
        write_char(12, col + i, ch, 0x2F); // white on green
    }
    crate::qemu_quit(crate::QEMU_PASS);
}
