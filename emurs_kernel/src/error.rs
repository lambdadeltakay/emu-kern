use alloc::string::String;
use core::panic::PanicInfo;

#[derive(Debug)]
pub struct EmuRsError {
    pub message: String,
}

#[cfg(feature = "embedded")]
#[panic_handler]
pub fn panic_handler(_info: &PanicInfo) -> ! {
    panic!()
}