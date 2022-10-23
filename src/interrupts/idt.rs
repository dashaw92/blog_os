use crate::{
    println,
    vga::{
        buffer::{set_color, DEFAULT_COLOR},
        colors::Color,
    },
};
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(doublefault_handler)
                .set_stack_index(super::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    set_color((Color::White, Color::Blue));
    println!("Breakpoint exception\n{:#?}", stack_frame);
    set_color(DEFAULT_COLOR);
}

extern "x86-interrupt" fn doublefault_handler(
    stack_frame: InterruptStackFrame,
    _err_code: u64,
) -> ! {
    panic!("Double fault exception!!!\n{:#?}", stack_frame);
}

#[cfg(test)]
mod tests {
    #[test_case]
    fn invokes_int3() {
        x86_64::instructions::interrupts::int3();
    }
}
