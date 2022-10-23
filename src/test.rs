#[cfg(test)]
pub(super) fn test_runner(tests: &[&dyn Fn()]) {
    use crate::vga::{
        buffer::WRITER,
        colors::{Color, ColorCode},
    };

    WRITER
        .lock()
        .set_color(ColorCode::of(Color::Yellow, Color::Black));
    crate::println!("Running {} tests", tests.len());
    for &test in tests {
        test();
    }
}
