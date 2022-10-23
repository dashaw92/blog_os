use pic8259::ChainedPics;
use spin::Mutex;

pub(super) const PIC_1_OFFSET: u8 = 32;
pub(super) const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub(super) static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

pub(super) fn init() {
    unsafe {
        PICS.lock().initialize();
    }
}
