use crate::EmuRsContext;
use alloc::rc::Rc;

pub trait EmuRsSubsystem {
    fn init(&mut self, context: Rc<EmuRsContext>);
}
