use crate::{
    disk::EmuRsDiskDriver,
    driver::EmuRsDriver,
    vfs::{EmuRsFilesystemManager, EmuRsFsDriver, EmuRsPath},
};
use alloc::boxed::Box;
use modular_bitfield::prelude::*;

// https://wiki.osdev.org/USTAR
// https://www.ibm.com/docs/en/aix/7.1?topic=files-tarh-file

pub struct EmuRsUstarFs {}

impl EmuRsUstarFs {}

impl EmuRsDriver for EmuRsUstarFs {
    fn name(&self) -> &str {
        todo!()
    }

    fn get_preference(&self) -> crate::driver::EmuRsDriverPreference {
        todo!()
    }

    fn get_claimed(&self) -> crate::device::EmuRsDevice {
        todo!()
    }

    fn setup_hardware(&self) {
        todo!()
    }
}

impl EmuRsFsDriver for EmuRsUstarFs {}
