#![no_std]
#![no_main]

use core::arch::naked_asm;

#[cfg(feature = "qemu")]
use rust_bare_riscv::qemu;
use rust_bare_riscv::some_c;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
#[unsafe(naked)]
pub extern "C" fn _start() -> ! {
    naked_asm!(
        "la sp, __stack_top",
        "call rust_main",
    )
}

#[unsafe(no_mangle)]
fn rust_main() -> ! {
    #[cfg(feature = "qemu")]
    {
        qemu::print("Enter n m: ");
        let n = qemu::read_u32();
        let m = qemu::read_u32();
        let result = some_c::gcd(n, m);
        qemu::print("gcd(");
        qemu::print_u32(n);
        qemu::print(", ");
        qemu::print_u32(m);
        qemu::print(") = ");
        qemu::print_u32(result);
        qemu::print("\n");
        qemu::exit(0);
    }

    #[cfg(not(feature = "qemu"))]
    {
        let _result = some_c::gcd(48, 18);
        loop {}
    }
}

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    #[cfg(feature = "qemu")]
    {
        use core::fmt::Write;
        let mut uart = qemu::Uart;
        let _ = writeln!(uart, "PANIC: {}", _info);
        qemu::exit(1);
    }

    #[cfg(not(feature = "qemu"))]
    loop {}
}
