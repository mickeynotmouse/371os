use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use spin::Mutex;
use pic8259::ChainedPics;
use pc_keyboard::{layouts, HandleControl, Keyboard, ScancodeSet1};
use core::sync::atomic::{AtomicUsize, Ordering};
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

static SCANCODE_QUEUE: Mutex<[u8; 64]> = Mutex::new([0u8; 64]);
static QUEUE_HEAD: AtomicUsize = AtomicUsize::new(0);
static QUEUE_TAIL: AtomicUsize = AtomicUsize::new(0);

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
            [InterruptIndex::Keyboard as usize]
            .set_handler_fn(keyboard_interrupt_handler);
        (*core::ptr::addr_of_mut!(IDT)).load();
    }
}

pub fn pop_scancode() -> Option<u8> {
    let head = QUEUE_HEAD.load(Ordering::Relaxed);
    let tail = QUEUE_TAIL.load(Ordering::Relaxed);
    if head == tail {
        return None;
    }
    let scancode = SCANCODE_QUEUE.lock()[head];
    QUEUE_HEAD.store((head + 1) % 64, Ordering::Relaxed);
    Some(scancode)
}

extern "x86-interrupt" fn breakpoint_handler(_stack_frame: InterruptStackFrame) {
    crate::serial_println!("EXCEPTION: BREAKPOINT");
    crate::println!("BREAKPOINT HIT");
}

extern "x86-interrupt" fn double_fault_handler(sf: InterruptStackFrame, _err: u64) -> ! {
    crate::serial_println!("DOUBLE FAULT: {:#?}", sf);
    crate::println!("DOUBLE FAULT");
    loop {}
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let scancode: u8 = unsafe { x86_64::instructions::port::Port::new(0x60).read() };

    let tail = QUEUE_TAIL.load(Ordering::Relaxed);
    let next_tail = (tail + 1) % 64;
    if next_tail != QUEUE_HEAD.load(Ordering::Relaxed) {
        SCANCODE_QUEUE.lock()[tail] = scancode;
        QUEUE_TAIL.store(next_tail, Ordering::Relaxed);
    }

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard as u8);
    }
}
