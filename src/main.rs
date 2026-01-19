#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

mod lib {
    mod some_c {
        include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    }

    pub fn gcd(n: u32, m: u32) -> u32 {
        let pair = &mut some_c::Pair { n, m };
        unsafe { some_c::gcd(pair) }
    }
}

#[cfg(target_os = "none")]
use core::arch::naked_asm;

#[cfg(target_os = "none")]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
#[unsafe(naked)]
pub extern "C" fn _start() -> ! {
    naked_asm!("la sp, __stack_top", "call main",)
}

#[cfg_attr(target_os = "none" , unsafe(no_mangle))]
fn main() {
    #[cfg(target_os = "none")]
    qemu::print("Starting\n");

    #[cfg(not(target_os = "none"))]
    println!("Starting");

    assert!(lib::gcd(48, 18) == 6);
    assert!(lib::gcd(100, 10) == 10);
    assert!(lib::gcd(7, 13) == 1);
    // assert!(lib::gcd(0, 5) == 5); // panics
    assert!(lib::gcd(12, 12) == 12);

    #[cfg(target_os = "none")]
    {
        qemu::print("Done\n");
        qemu::exit(0);
    }

    #[cfg(not(target_os = "none"))]
    println!("Done");
}

#[cfg(target_os = "none")]
mod qemu {
    const UART_ADDR: usize = 0x10000000;
    const VIRT_TEST: usize = 0x100000;

    pub struct Uart;

    impl core::fmt::Write for Uart {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            print(s);
            Ok(())
        }
    }

    pub fn print(s: &str) {
        for byte in s.bytes() {
            unsafe { (UART_ADDR as *mut u8).write_volatile(byte) };
        }
    }

    pub fn exit(code: u32) -> ! {
        let exit_code = if code == 0 {
            0x5555
        } else {
            (code << 16) | 0x3333
        };
        unsafe { (VIRT_TEST as *mut u32).write_volatile(exit_code) };
        loop {}
    }
}

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use core::fmt::Write;
    let mut uart = qemu::Uart;
    let _ = writeln!(uart, "Panic: {}", info);
    qemu::exit(1);
}
