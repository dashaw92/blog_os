use core::fmt::{self, Write};
use core::ops::RangeInclusive;

use crate::vga::colors::*;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

const VGA_ADDR_START: usize = 0xB8000;
const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column: usize,
    color: ColorCode,
    buffer: &'static mut Buffer,
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new());
}

impl Writer {
    fn new() -> Self {
        Self {
            column: 0,
            color: ColorCode::of(Color::LightGray, Color::Blue),
            buffer: unsafe { &mut *(VGA_ADDR_START as *mut Buffer) },
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column;

                let color = self.color;
                self.buffer.chars[row][col].write(ScreenChar { ascii: byte, color });
                self.column += 1;
            }
        }
    }

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
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii: b' ',
            color: self.color,
        };

        for col in 0..BUFFER_WIDTH {
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

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}
