use alloc::rc::Rc;
use tinyvec::ArrayVec;
use crate::EmuRsContext;
use alloc::vec::Vec;

/// Compiled in program for the operating system
pub trait EmuRsProgram: Sync {
    fn new(os_context: Rc<EmuRsContext>) -> Self
    where
        Self: Sized;
    fn required_firmware() -> Vec<&'static str> {
        return Vec::new();
    }
    fn init(&mut self);
    fn step(&mut self);
    fn vsync(&mut self);
    fn exit(&mut self);
}
