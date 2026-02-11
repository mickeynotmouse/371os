# osirs — CS371 RISC-V OS

## Build

First, add the RISC-V target:

rustup target add riscv64imac-unknown-none-elf

Then, build the crate with the linker scripts:

cargo rustc --target riscv64imac-unknown-none-elf -- -C link-arg=-Tmemory.x -C link-arg=-Tlink.x -C panic=abort

The compiled binary will be located at target/riscv64imac-unknown-none-elf/debug/osirs

Rename it for QEMU:

cp target/riscv64imac-unknown-none-elf/debug/osirs main

---

## Run

Run the kernel in QEMU with:

qemu-system-riscv64 -machine sifive_u -bios none -kernel main

Should open blank window — this is expected because the kernel loops infinitely.

---

## Kernel behavior

Entry point is `_start` in src/main.rs  
Infinite loop keeps the CPU busy  
Panic handler loops infinitely

---

## Checklist

- _start function added  
- #[panic_handler] defined  
- memory.x and link.x linker scripts included  
- Built for riscv64imac-unknown-none-elf  
- Successfully run in QEMU
