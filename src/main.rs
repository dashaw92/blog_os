#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"dashaw92 was here";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_ptr = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_ptr.offset(i as isize * 2) = byte;
            *vga_ptr.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
