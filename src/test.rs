use crate::{serial_print, serial_println};

#[cfg(test)]
pub(super) fn test_runner(tests: &[&dyn Testable]) {
    use crate::qemu::*;

    serial_println!("Running {} test(s)", tests.len());
    for &test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

pub trait Testable {
    fn run(&self);
}

impl<T: Fn()> Testable for T {
    fn run(&self) {
        serial_print!("Test \"{}\"...\t", core::any::type_name::<T>());
        (self)();
        serial_println!("[passed]");
    }
}

#[cfg(test)]
mod tests {
    /// This test is introduced as the first unit test to demonstrate
    /// the working test runner implementation. I'm leaving it here
    /// not as a unit test on the whole project, but rather as a test
    /// for the test runner itself. If this fails, there's most likely
    /// something wrong with the runner, and the proceeding failing
    /// tests should be taken with a grain of skepticism.
    #[test_case]
    fn never_fails() {
        assert_eq!(1, 1);
    }
}
