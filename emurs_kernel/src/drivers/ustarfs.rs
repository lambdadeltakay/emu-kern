
use crate::{
    driver::EmuRsDriver,
    vfs::{EmuRsFsDriver},
};




// https://wiki.osdev.org/USTAR
// https://www.ibm.com/docs/en/aix/7.1?topic=files-tarh-file

#[derive(Default)]
pub struct EmuRsUstarFs;

impl EmuRsUstarFs {}

impl EmuRsDriver for EmuRsUstarFs {
    fn name(&self) -> &'static str {
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
