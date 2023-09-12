use alloc::string::String;
use core::panic::PanicInfo;

#[derive(Debug)]
pub struct EmuRsError<'owner> {
    pub message: &'owner str,
}

#[cfg(feature = "embedded")]
#[panic_handler]
pub fn panic_handler(_info: &PanicInfo) -> ! {
    panic!()
}
