use alloc::string::String;
use core::panic::PanicInfo;

#[derive(Debug)]
pub enum EmuRsErrorReason<'owner> {
    Custom(&'owner str),
    Unknown,
    OperationNotSupported,
    InvalidPath,
    EndOfDiskHit,
}

#[derive(Debug)]
pub struct EmuRsError<'owner> {
    pub reason: EmuRsErrorReason<'owner>,
}

#[cfg(feature = "embedded")]
#[panic_handler]
pub fn panic_handler(info: &PanicInfo) -> ! {
    loop {}
}
