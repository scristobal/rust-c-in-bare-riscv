Cross-compile Rust+C into RISC-V baremetal

sudo pacman -S riscv64-elf-gcc qemu-system-riscv
cargo build --release --features qemu
qemu-system-riscv64 -machine virt -nographic -bios none -kernel target/riscv64gc-unknown-none-elf/release/rust-bare-riscv

