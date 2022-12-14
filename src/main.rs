#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod interrupts;
mod qemu;
mod serial;
#[cfg(test)]
mod test;
mod vga;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    halt();
}

fn init() {
    interrupts::init_all();
}

/// Executes HLT in an endless loop- instead of the CPU spinning endlessly,
/// the CPU is put into a "sleep state" where it consumes MUCH less energy.
pub fn halt() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::vga::{buffer::set_color, colors::Color};
    set_color((Color::White, Color::Red));

    println!("PANIC: {}", info);

    halt();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::qemu::{exit_qemu, QemuExitCode};

    serial_println!("TEST PANIC: {}", info);
    exit_qemu(QemuExitCode::Failed);
}
