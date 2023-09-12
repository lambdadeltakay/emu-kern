use alloc::string::String;

use crate::device::EmuRsDevice;

/// Type that all drivers must implement for a future driver manager
pub trait EmuRsDriver {
    /// The name of the driver
    fn name(&self) -> &str;
    /// The devices this driver would like to claim. 
    /// Currently completely unimplemented
    fn get_claimed(&self) -> EmuRsDevice;
    /// Initialize the claimed hardware
    fn setup_hardware(&self);
}
