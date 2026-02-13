use core::mem::size_of;

pub const SIZE: usize = 0x80;
static mut BUS: [u8; SIZE] = [0; SIZE];

//mast bytes (1 bit per data byte)
const MASK_BYTES: usize = SIZE / 8;
const DATA_START: usize = MASK_BYTES;

fn init() {
    unsafe {
        //power of two
        assert!(SIZE & (SIZE - 1) == 0);

        //mask must mark itself as used
        for i in 0..MASK_BYTES {
            BUS[i] = 0xFF;
        }

        //zero data
        for i in DATA_START..SIZE {
            BUS[i] = 0;
        }
    }
}

//test if byte is used
fn used(i: usize) -> bool {
    unsafe {
        BUS[i] != 0
    }
}

//mark byte as used/free
fn set_used(i: usize, val: bool) {
    unsafe {
        BUS[i] = if val {1}
        else {0}; 
    }
}

//return offset in BUS
pub fn malloc(s: usize) -> Option<usize> {
    unsafe {
        //lazy init if first byte not defined
        if BUS[0] == 0 {
            init();
        }

        if s == 0 || s > SIZE - DATA_START {
            return None;
        }

        //scan for contiguous free region
        let mut run = 0usize;
        let mut start = DATA_START;

        for i in DATA_START..SIZE {
            if !used(i) {
                if run == 0 {
                    start = i;
                }
                run += 1;

                if run == s {
                    for j in start..start + s {
                        set_used(j, true);
                    }
                    return Some(start);
                }
            }
            else {
                run = 0;
            }
        }
    }
    None
}

//assign arbitrary type
pub fn setter<T>(val: T, loc: usize) {
    unsafe {
        let src = &val as *const T as *const u8;
        for i in 0..size_of::<T>() {
            BUS[loc + i] = *src.add(i);
        }
    }
}

// arbitrary type out
pub fn getter<T>(loc: usize) -> T {
    unsafe{
        let mut out: T = core::mem::zeroed();
        let dst = &mut out as *mut T as *mut u8;

        for i in 0..size_of::<T>() {
            *dst.add(i) = BUS[loc + i];
        }

        out
    }
}

