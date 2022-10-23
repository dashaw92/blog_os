#![allow(dead_code)]

use core::fmt::{self, Write};
use core::ops::RangeInclusive;

use crate::vga::colors::*;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;
use x86_64::instructions::interrupts;

const VGA_ADDR_START: usize = 0xB8000;
pub const VGA_WIDTH: usize = 80;
pub const VGA_HEIGHT: usize = 25;
pub const DEFAULT_COLOR: ColorCode = ColorCode::of(Color::LightGray, Color::Black);

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; VGA_WIDTH]; VGA_HEIGHT],
}

/// Wraps the VGA text buffer in memory to provide printing capabilities
pub struct Writer {
    column: usize,
    color: ColorCode,
    buffer: &'static mut Buffer,
}

lazy_static! {
    /// Global instance of Writer
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new());
}

impl Writer {
    fn new() -> Self {
        Self {
            column: 0,
            color: DEFAULT_COLOR,
            buffer: unsafe { &mut *(VGA_ADDR_START as *mut Buffer) },
        }
    }

    /// Sets the current color of the output
    pub fn set_color(&mut self, color: ColorCode) {
        self.color = color;
    }

    /// Writes a single `u8` to the internal VGA buffer
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column >= VGA_WIDTH {
                    self.new_line();
                }

                let row = VGA_HEIGHT - 1;
                let col = self.column;

                let color = self.color;
                self.buffer.chars[row][col].write(ScreenChar { ascii: byte, color });
                self.column += 1;
            }
        }
    }

    /// Writes a string, byte by byte, to the internal VGA buffer
    pub fn write_string(&mut self, s: &str) {
        const ASCII_TABLE: RangeInclusive<u8> = 0x20..=0x7E;

        s.bytes()
            .map(|b| {
                if ASCII_TABLE.contains(&b) || b == b'\n' {
                    b
                } else {
                    0xFE
                }
            })
            .for_each(|b| self.write_byte(b));
    }

    fn new_line(&mut self) {
        for row in 1..VGA_HEIGHT {
            for col in 0..VGA_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(VGA_HEIGHT - 1);
        self.column = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii: b' ',
            color: DEFAULT_COLOR,
        };

        for col in 0..VGA_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

/// Convenience macro for printing to the VGA buffer
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::buffer::_print(format_args!($($arg)*)));
}

/// Convenience macro for printing to the VGA buffer with an added newline (`\n`)
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

/// Sets the color of the global VGA buffer `Writer`
pub fn set_color<C: Into<ColorCode>>(c: C) {
    interrupts::without_interrupts(|| {
        WRITER.lock().set_color(c.into());
    });
}

#[cfg(test)]
mod tests {
    use crate::vga::buffer::{VGA_HEIGHT, WRITER};

    #[test_case]
    fn test_println_simple() {
        println!("test_println_simple output");
    }

    #[test_case]
    fn test_println_many() {
        (1..=200).for_each(|x| println!("Testing print {}", x));
    }

    #[test_case]
    fn ensure_printing_works() {
        let s = "Some test string that fits on a single line";
        println!("{}", s);
        for (i, c) in s.chars().enumerate() {
            let s_char = WRITER.lock().buffer.chars[VGA_HEIGHT - 2][i].read();
            assert_eq!(char::from(s_char.ascii), c);
        }
    }
}
