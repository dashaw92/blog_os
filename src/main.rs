#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod qemu;
mod serial;
mod test;
mod vga;

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::vga::{buffer::set_color, colors::Color};
    set_color((Color::White, Color::Red));

    println!("PANIC: {}", info);

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::qemu::{exit_qemu, QemuExitCode};

    serial_println!("TEST PANIC: {}", info);
    exit_qemu(QemuExitCode::Failed);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    panic!("Halting!");
}
