Bare-metal Rust on RISC-V 64

Key things to remember:

- -march=rv64gc -mabi=lp64d: ABI must match between Rust and C.
  rv64gc = 64-bit + IMAFD + compressed extensions.
  lp64d = 64-bit pointers, doubles in FP registers.
  Mismatch = broken calling convention between Rust and C.

- link.x: No OS = no default memory layout. Tells linker where
  code goes (0x80000000 = QEMU virt RAM), defines stack location,
  exports symbols (__stack_top, __bss_start) for asm startup code.

- naked _start in .text.entry: Rust functions assume valid stack.
  Must set SP before any Rust runs. Naked = no prologue.
  .text.entry section = linker puts it first at 0x80000000.

- #![no_std] + #![no_main] + panic_handler: no_std removes libc.
  no_main skips Rust runtime init. panic_handler mandatory because
  std normally provides it.

- CC_riscv64gc_unknown_none_elf in .cargo/config.toml: build.rs
  runs on host but cc/bindgen must invoke cross compiler, not host gcc.

- 0x80000000: Where QEMU virt loads kernel. Hardware-specific.

Setup: sudo pacman -S riscv64-elf-gcc qemu-system-riscv
Build: cargo build --release --features qemu
Run:   qemu-system-riscv64 -machine virt -nographic -bios none -kernel target/riscv64gc-unknown-none-elf/release/rust-bare-riscv
