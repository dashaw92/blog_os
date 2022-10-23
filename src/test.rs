#[cfg(test)]
pub(super) fn test_runner(tests: &[&dyn Fn()]) {
    use crate::qemu::*;
    use crate::serial_println as println;

    println!("Running {} test(s)", tests.len());
    for &test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

#[cfg(test)]
mod tests {

    use crate::serial_print as print;
    use crate::serial_println as println;

    #[test_case]
    fn trivial() {
        print!("Trivial assertion... ");
        assert_eq!(1, 2);
        println!("[ok]");
    }
}
