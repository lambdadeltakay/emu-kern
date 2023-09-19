use alloc::string::String;
use core::panic::PanicInfo;

#[derive(Clone, Debug)]
pub enum EmuRsErrorReason {
    Custom(String),
    Unknown,
    OperationNotSupported,
    InvalidPath,
    EndOfDiskHit,
}

#[derive(Clone, Debug)]
pub struct EmuRsError {
    pub reason: EmuRsErrorReason,
}

#[cfg(feature = "embedded")]
#[panic_handler]
pub fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
