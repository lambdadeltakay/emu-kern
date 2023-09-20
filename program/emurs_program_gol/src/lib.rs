#![no_std]

extern crate alloc;

use alloc::rc::Rc;
use emurs_kernel::prelude::nalgebra::DMatrix;
use emurs_kernel::program::EmuRsProgram;
use emurs_kernel::video::{EmuRsColorFormatGrey1, EmuRsGreyColor, EmuRsTexture};
use emurs_kernel::EmuRsContext;

pub struct Program {
    gol_board: DMatrix<bool>,
}

impl EmuRsProgram for Program {
    fn new() -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn step(&mut self, os_context: &EmuRsContext) {
        todo!()
    }

    fn vsync(&mut self, os_context: &EmuRsContext) {
        let texture = EmuRsTexture::new(self.gol_board.map(|element| {
            return EmuRsColorFormatGrey1::new(element as u8);
        }));
    }

    fn exit(&mut self) {
        todo!()
    }
}
