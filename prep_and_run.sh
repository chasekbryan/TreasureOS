#!/usr/bin/env bash
set -euo pipefail

# 1. Check prerequisites
for cmd in rustup cargo qemu-system-i386 grub-mkrescue; do
  if ! command -v "$cmd" &>/dev/null; then
    echo "Error: '$cmd' not found. Please install it first." >&2
    exit 1
  fi
done

# 2. Use nightly and pull in rust-src (needed for build-std)
echo "▶ Setting Rust toolchain to nightly and installing rust-src..."
rustup override set nightly
rustup component add rust-src --toolchain nightly

# 3. Generate .cargo/config.toml if missing
if [ ! -f .cargo/config.toml ]; then
  echo "▶ Creating .cargo/config.toml for build-std and panic=abort..."
  mkdir -p .cargo
  cat > .cargo/config.toml << 'EOF'
[build]
target = "i686-unknown-none.json"
rustflags = [
  "-Zbuild-std=core,alloc",
  "-Zbuild-std-features=compiler-builtins-mem",
  "-C", "panic=abort",
]
EOF
fi

# 4. Compile the kernel
echo "▶ Building TreasureOS (release)..."
cargo clean
cargo +nightly build --release --target i686-unknown-none.json

# 5. Prepare ISO tree
echo "▶ Assembling ISO directory..."
rm -rf isofs
mkdir -p isofs/boot/grub
cp target/i686-unknown-none/release/treasureos isofs/boot/treasureos.bin

cat > isofs/boot/grub/grub.cfg << 'EOF'
set timeout=0
set default=0

menuentry "TreasureOS v0.1" {
    multiboot /boot/treasureos.bin
    boot
}
EOF

# 6. Create the ISO
echo "▶ Generating treasureos.iso..."
i686-elf-grub-mkrescue -o treasureos.iso isofs

# 7. Launch under QEMU
echo "▶ Launching TreasureOS in QEMU..."
qemu-system-i386 -cdrom treasureos.iso -serial stdio

