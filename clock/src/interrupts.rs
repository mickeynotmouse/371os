use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use spin::Mutex;
use pic8259::ChainedPics;
use pc_keyboard::{layouts, HandleControl, Keyboard, ScancodeSet1};
use core::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use crate::gdt;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard = PIC_1_OFFSET + 1,
}

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub static mut KB_RAW: Keyboard<layouts::Us104Key, ScancodeSet1> = Keyboard::new(
    ScancodeSet1::new(),
    layouts::Us104Key,
    HandleControl::Ignore,
);

// Timer state
pub static TICKS: AtomicUsize = AtomicUsize::new(0);
pub static START_TICKS: AtomicUsize = AtomicUsize::new(0);
pub static CLOCK_RUNNING: AtomicBool = AtomicBool::new(false);

// Input state
pub static DIGIT_COUNT: AtomicUsize = AtomicUsize::new(0);
pub static INPUT_DIGITS: Mutex<[u8; 6]> = Mutex::new([0u8; 6]);

pub fn init_idt() {
    unsafe {
        (*core::ptr::addr_of_mut!(IDT))
            .breakpoint
            .set_handler_fn(breakpoint_handler);
        (*core::ptr::addr_of_mut!(IDT))
            .double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        (&mut (*core::ptr::addr_of_mut!(IDT)))
            [InterruptIndex::Timer as usize]
            .set_handler_fn(timer_handler);
        (&mut (*core::ptr::addr_of_mut!(IDT)))
            [InterruptIndex::Keyboard as usize]
            .set_handler_fn(keyboard_interrupt_handler);
        (*core::ptr::addr_of_mut!(IDT)).load();
    }
}

extern "x86-interrupt" fn breakpoint_handler(_stack_frame: InterruptStackFrame) {
    crate::serial_println!("EXCEPTION: BREAKPOINT");
}

extern "x86-interrupt" fn double_fault_handler(sf: InterruptStackFrame, _err: u64) -> ! {
    crate::serial_println!("DOUBLE FAULT: {:#?}", sf);
    crate::println!("DOUBLE FAULT");
    loop {}
}

extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame) {
    TICKS.fetch_add(1, Ordering::Relaxed);
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer as u8);
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let scancode: u8 = unsafe { x86_64::instructions::port::Port::new(0x60).read() };

    if let Ok(Some(key_event)) = unsafe { (*core::ptr::addr_of_mut!(KB_RAW)).add_byte(scancode) } {
        if let Some(key) = unsafe { (*core::ptr::addr_of_mut!(KB_RAW)).process_keyevent(key_event) } {
            if !CLOCK_RUNNING.load(Ordering::Relaxed) {
                match key {
                    pc_keyboard::DecodedKey::Unicode(c) => {
                        match c {
                            '0'..='9' => {
                                let count = DIGIT_COUNT.load(Ordering::Relaxed);
                                if count < 6 {
                                    INPUT_DIGITS.lock()[count] = c as u8 - b'0';
                                    DIGIT_COUNT.fetch_add(1, Ordering::Relaxed);
                                    crate::print!("{}", c);
                                    if count + 1 == 6 {
                                        START_TICKS.store(TICKS.load(Ordering::Relaxed), Ordering::Relaxed);
                                        CLOCK_RUNNING.store(true, Ordering::Relaxed);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard as u8);
    }
}
