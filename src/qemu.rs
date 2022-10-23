#![allow(dead_code)]

/// Address of qemu ISA debug exit device (Cargo.toml)
const ISA_DEBUG_IOBASE: u16 = 0xF4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    /// Execution was successful - no problems
    Success = 0x10,
    /// Execution failed
    Failed = 0x11,
}

/// Exits QEMU with the provided exit code
pub fn exit_qemu(code: QemuExitCode) -> ! {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(ISA_DEBUG_IOBASE);
        port.write(code as u32);
    }

    unreachable!("QEMU will have exited by this point.")
}
