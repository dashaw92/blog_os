use crate::vga::buffer::set_color;
pub(self) use crate::{print, println};

#[cfg(test)]
pub(super) fn test_runner(tests: &[&dyn Fn()]) {
    use crate::vga::{buffer::DEFAULT_COLOR, colors::Color};

    set_color((Color::Yellow, Color::Black));
    println!("Running {} tests", tests.len());
    set_color((Color::Black, Color::White));
    for &test in tests {
        test();
    }

    set_color(DEFAULT_COLOR);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn trivial() {
        print!("Trivial assertion... ");
        assert_eq!(1, 1);
        println!("[ok]");
    }
}
