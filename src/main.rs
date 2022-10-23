#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
mod test;

mod vga;

use core::panic::PanicInfo;

use crate::vga::colors::{Color, ColorCode};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga::buffer::WRITER
        .lock()
        .set_color(ColorCode::of(Color::White, Color::Red));
    println!("PANIC: {}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    panic!("Halting!");
}
