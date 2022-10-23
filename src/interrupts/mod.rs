/// Global descriptor table
pub(crate) mod gdt;
/// Interrupt descriptor table
pub(crate) mod idt;
/// PIC implementation
pub(crate) mod pic;

/// Initializes all child modules in the correct order
pub(crate) fn init_all() {
    gdt::init();
    idt::init();
    pic::init();
}
