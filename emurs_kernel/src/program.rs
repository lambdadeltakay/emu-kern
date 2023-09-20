use crate::EmuRsContext;
use alloc::rc::Rc;
use alloc::vec::Vec;
use tinyvec::ArrayVec;

/// Compiled in program for the operating system
pub trait EmuRsProgram: Sync {
    fn new() -> Self
    where
        Self: Sized;
    fn required_firmware() -> Vec<&'static str> {
        return Vec::new();
    }
    fn step(&mut self, os_context: &EmuRsContext);
    fn vsync(&mut self, os_context: &EmuRsContext);
    fn exit(&mut self);
}
