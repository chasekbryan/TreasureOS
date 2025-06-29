// src/main.rs — TreasureOS Kernel
// Testing Instructions:
// 1. Install nightly Rust and required components:
//      rustup default nightly
//      rustup component add rust-src llvm-tools-preview
// 2. Install the `bootimage` tool:
//      cargo install bootimage
// 3. Create `i686-unknown-none.json` and `linker.ld` as specified in project docs.
// 4. Configure Cargo in `.cargo/config.toml`:
//      [unstable]
//      build-std = ["core","compiler_builtins"]
//      build-std-features = ["compiler-builtins-mem"]
//      [build]
//      target = "i686-unknown-none.json"
//      rustflags = ["-C","link-arg=-Tlinker.ld"]
//      [target.i686-unknown-none]
//      runner = "qemu-system-i386 -kernel"
// 5. Build a bootable ISO:
//      cargo bootimage --release
// 6. Run under QEMU:
//      qemu-system-i386 \
//        -cdrom target/bootimage/treasureos.iso \
//        -serial stdio -display none

#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::arch::{asm, global_asm};
use core::panic::PanicInfo;

// Multiboot header for GRUB
global_asm!(r#"
    .section .multiboot
    .long 0x1BADB002       /* magic */
    .long 0x00000003       /* flags: align + mem_info only */
    .long -(0x1BADB002 + 0x00000003)
"#);

/// Kernel entry point (called by bootloader)
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::clear_screen();
    vga::print("TreasureOS v0.1\n");

    gdt::init();
    idt::init();
    paging::init();
    memory::init();
    pit::init(1000);       // 1 kHz timer
    scheduler::init();
    keyboard::init();
    disk::init();
    syscall::init();

    vga::print("Initialization complete.\n");
    loop { unsafe { asm!("hlt"); } }
}

/// Panic handler—halts on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    vga::print("KERNEL PANIC!\n");
    loop { unsafe { asm!("hlt"); } }
}

// --- VGA Text Mode ---
mod vga {
    const BUFFER: *mut u16 = 0xb8000 as *mut u16;
    static mut X: usize = 0;
    static mut Y: usize = 0;

    fn advance_cursor() {
        unsafe {
            X = (X + 1) % 80;
            if X == 0 { Y = (Y + 1) % 25; }
        }
    }

    /// Clear the screen
    pub fn clear_screen() {
        unsafe {
            for row in 0..25 {
                for col in 0..80 {
                    BUFFER.add(row*80+col)
                        .write_volatile((0x07 << 8) | b' ' as u16);
                }
            }
            X = 0;
            Y = 0;
        }
    }

    /// Print a string
    pub fn print(s: &str) {
        for b in s.bytes() {
            match b {
                b'\n' => unsafe { X = 0; Y = (Y + 1) % 25; },
                byte    => unsafe {
                    BUFFER.add(Y*80+X)
                        .write_volatile((0x07 << 8) | byte as u16);
                    advance_cursor();
                }
            }
        }
    }
}

// --- Global Descriptor Table ---
mod gdt {
    /// Initialize GDT (stub)
    pub fn init() {}
}

// --- Interrupt Descriptor Table ---
mod idt {
    /// Initialize IDT (stub)
    pub fn init() {}
}

// --- Paging ---
mod paging {
    /// Initialize paging (stub)
    pub fn init() {}
}

// --- Physical Memory Management ---
mod memory {
    static mut NEXT_FRAME: usize = 0x0100_0000;

    pub fn init() {}

    pub fn alloc(size: usize) -> *mut u8 {
        unsafe {
            let addr = NEXT_FRAME;
            NEXT_FRAME += size;
            addr as *mut u8
        }
    }
}

// --- Programmable Interval Timer ---
mod pit {
    pub fn init(_freq_hz: u32) {}
}

// --- Scheduler ---
mod scheduler {
    pub fn init() {}
    pub fn tick() {}
}

// --- Keyboard Driver ---
mod keyboard {
    pub fn init() {}
}

// --- Disk Driver ---
mod disk {
    pub fn init() {}
}

// --- System Call Interface ---
mod syscall {
    pub fn init() {}
}
