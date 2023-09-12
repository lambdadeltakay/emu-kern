use alloc::string::String;

use crate::device::EmuRsDevice;

pub trait EmuRsDriver {
    fn name(&self) -> &str;
    fn get_claimed(&self) -> EmuRsDevice;
    fn setup_hardware(&self);
}
