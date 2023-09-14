use crate::driver::EmuRsDriverPreference;
use crate::vfs::EmuRsFilesystemManager;
use crate::{device::EmuRsDevice, error::EmuRsError};
use alloc::collections::BTreeMap;
use tinyvec::TinyVec;

use crate::{
    driver::EmuRsDriver,
    vfs::{EmuRsFsDriver, EmuRsPath},
};

// Mounted rom database
pub struct EmuRsGameFs<'owner> {
    pub search_paths: TinyVec<[EmuRsPath<'owner>; 2]>,
}

impl<'owner> EmuRsDriver for EmuRsGameFs<'owner> {
    fn name(&self) -> &str {
        return "Game Filesystem";
    }

    fn get_preference(&self) -> EmuRsDriverPreference {
        todo!()
    }

    fn get_claimed(&self) -> EmuRsDevice {
        todo!()
    }

    fn setup_hardware(&self) {
        todo!()
    }
}

impl<'owner> EmuRsFsDriver for EmuRsGameFs<'owner> {
    fn read(
        &self,
        vfs: &mut EmuRsFilesystemManager,
        file: EmuRsPath,
        buffer: &mut [u8],
        offset: usize,
    ) -> Result<(), EmuRsError> {
        return Ok(());
    }
}
