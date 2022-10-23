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

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub(super) enum InterruptIdx {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIdx {
    pub(super) fn as_u8(self) -> u8 {
        self as u8
    }

    pub(super) fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

impl From<InterruptIdx> for u8 {
    fn from(value: InterruptIdx) -> Self {
        value.as_u8()
    }
}

impl From<InterruptIdx> for usize {
    fn from(value: InterruptIdx) -> Self {
        value.as_usize()
    }
}

pub(super) fn send_eoi(idx: InterruptIdx) {
    unsafe {
        PICS.lock().notify_end_of_interrupt(idx.into());
    }
}
