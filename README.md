# TreasureOS Kernel

**Version:** v0.1 (Bare‐bones Multiboot‑compliant Rust kernel)

---

## Overview

TreasureOS is a minimal monolithic kernel written in Rust, targeting 32‑bit x86 (`i386`) and bootable via GRUB. It provides a foundation for:

* A VGA text‑mode console driver
* Multiboot header for GRUB compatibility
* Skeleton modules for GDT, IDT, paging, memory management, PIT timer, scheduler, drivers, and syscalls

This README covers how to build, package, and run TreasureOS under QEMU.

prep_and_run.sh should set everything up appropriately for the first test run upon a new download. 

---

## Prerequisites

1. **Rust nightly toolchain** with source and LLVM tools:

   ```bash
   rustup default nightly
   rustup component add rust-src llvm-tools-preview
   ```
2. **i686‑unknown‑none JSON target spec** at project root: `i686-unknown-none.json`
3. **Linker script** at project root: `linker.ld` (see `src/` for example)
4. **GRUB toolchain** installed via Homebrew:

   ```bash
   brew install i686-elf-grub    # provides i686-elf-grub-mkrescue
   ```
5. **QEMU** for running the ISO:

   ```bash
   brew install qemu
   ```

---

## Building the Kernel

1. Clean previous artifacts:

   ```bash
   cargo clean
   ```
2. Compile for your custom target:

   ```bash
   cargo build --release --target i686-unknown-none.json
   ```

   The resulting ELF will be at:

   ```
   target/i686-unknown-none/release/treasureos
   ```

---

## Packaging as a Bootable ISO

1. Prepare the ISO directory tree:

   ```bash
   rm -rf isofs treasureos.iso
   mkdir -p isofs/boot/grub
   ```
2. Copy the kernel into the GRUB path:

   ```bash
   cp target/i686-unknown-none/release/treasureos isofs/boot/treasureos.bin
   ```
3. Create a minimal `grub.cfg`:

   ```bash
   cat > isofs/boot/grub/grub.cfg <<EOF
   set timeout=0
   set default=0

   menuentry "TreasureOS v0.1" {
       multiboot /boot/treasureos.bin
       boot
   }
   EOF
   ```
4. Build the ISO with GRUB:

   ```bash
   i686-elf-grub-mkrescue -o treasureos.iso isofs
   ```

---

## Running under QEMU

Launch the ISO in QEMU with VGA output:

```bash
qemu-system-i386 \
  -cdrom treasureos.iso \
  -serial stdio
```

You should see within the QEMU window:

```
TreasureOS v0.1
Initialization complete.
```

---

## Project Structure

```
├── Cargo.toml
├── linker.ld
├── i686-unknown-none.json
├── src
│   └── main.rs       # Kernel entry, VGA driver, module stubs
├── isofs             # ISO build directory (generated)
└── treasureos.iso    # Bootable ISO (generated)
```

---

## Next Steps

* **GDT/IDT**: flesh out the descriptor tables and interrupt handlers
* **Paging**: implement page tables and enable paging
* **Memory allocator**: replace bump allocator with frame allocator using Multiboot memory map
* **Scheduler**: implement context switching on PIT ticks
* **Syscalls**: define syscall ABI and handlers
* **Drivers**: add keyboard scan code decoding, disk I/O, and more

Contributions and experiments are welcome!
