

use crate::device::EmuRsDevice;
use crate::EmuRsContext;
use alloc::rc::Rc;


/// Type that all drivers must implement for a future driver manager
pub trait EmuRsDriver {
    /// The name of the driver
    fn name(&self) -> &'static str;
    /// How likely this is to be used on this system
    fn get_preference(&mut self) -> EmuRsDriverPreference;
    /// The devices this driver would like to claim.
    /// Currently completely unimplemented
    fn get_claimed(&mut self) -> EmuRsDevice;
    /// Initialize the claimed hardware
    fn init(&mut self, _context: Rc<EmuRsContext>) {}
}

pub enum EmuRsDriverPreference {
    Forbidden,
    Fallback,
    Suboptimal,
    Preferred,
}
