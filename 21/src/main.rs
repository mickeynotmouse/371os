use std::io::Write;

fn main() {
    unsafe {
        let hw_i32: [i32; 3] = [0x6C6C6548, 0x6F77206F, 0x21646C72];
        let hw_bytes: &[u8; 12] = std::mem::transmute(&hw_i32);
        std::io::stdout().write_all(hw_bytes).unwrap();
    }
}

