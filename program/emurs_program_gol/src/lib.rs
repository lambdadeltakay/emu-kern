#![no_std]

extern crate alloc;

use emurs_kernel::program::EmuRsProgram;

pub struct Program;

impl EmuRsProgram for Program {
    fn new(os_context: alloc::rc::Rc<emurs_kernel::EmuRsContext>) -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn init(&mut self) {
        todo!()
    }

    fn step(&mut self) {
        todo!()
    }

    fn vsync(&mut self) {
        todo!()
    }

    fn exit(&mut self) {
        todo!()
    }
}
