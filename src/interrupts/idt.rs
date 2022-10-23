use crate::{
    interrupts::pic::{self, InterruptIdx},
    print, println,
    vga::{
        buffer::{set_color, DEFAULT_COLOR},
        colors::Color,
    },
};
use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::{
    instructions::port::Port,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(doublefault_handler)
                .set_stack_index(super::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt[InterruptIdx::Timer.into()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIdx::Keyboard.into()].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

pub(super) fn init() {
    IDT.load();
}

/// Handles breakpoint exceptions
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    set_color((Color::White, Color::Blue));
    println!("Breakpoint exception\n{stack_frame:#?}");
    set_color(DEFAULT_COLOR);
}

/// Handles double faults
extern "x86-interrupt" fn doublefault_handler(
    stack_frame: InterruptStackFrame,
    _err_code: u64,
) -> ! {
    panic!("Double fault exception!!!\n{stack_frame:#?}");
}

/// Handles PIC timer interrupts
extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    print!(".");
    pic::send_eoi(InterruptIdx::Timer);
}

/// Handles keyboard! :D
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    const PS2_CTRL_PORT: u16 = 0x60;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
            Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
        );
    }

    let mut keyb = KEYBOARD.lock();
    let mut port = Port::new(PS2_CTRL_PORT);

    let scancode: u8 = unsafe { port.read() };
    keyb.add_byte(scancode)
        .into_iter()
        .flatten() //Iter<Option<T>> -> Iter<T>
        .flat_map(|event| keyb.process_keyevent(event)) //pass valid scancodes to the keyboard
        .for_each(|key| match key {
            DecodedKey::Unicode(ch) => print!("{ch}"),
            DecodedKey::RawKey(raw) => print!("{raw:?}"),
        });

    pic::send_eoi(InterruptIdx::Keyboard);
}

#[cfg(test)]
mod tests {
    #[test_case]
    fn invokes_int3() {
        x86_64::instructions::interrupts::int3();
    }
}
