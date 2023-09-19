use crate::EmuRsContext;
use crate::{
    disk::EmuRsDiskDriver,
    driver::EmuRsDriver,
    vfs::{EmuRsFilesystemManager, EmuRsFsDriver, EmuRsPath},
};
use alloc::rc::Rc;
use core::cell::RefCell;
use modular_bitfield::prelude::*;

// https://wiki.osdev.org/USTAR
// https://www.ibm.com/docs/en/aix/7.1?topic=files-tarh-file

#[derive(Default)]
pub struct EmuRsUstarFs;

impl EmuRsUstarFs {}

impl EmuRsDriver for EmuRsUstarFs {
    fn name(&self) -> &str {
        todo!()
    }

    fn get_preference(&mut self) -> crate::driver::EmuRsDriverPreference {
        todo!()
    }

    fn get_claimed(&mut self) -> crate::device::EmuRsDevice {
        todo!()
    }
}

impl EmuRsFsDriver for EmuRsUstarFs {}
