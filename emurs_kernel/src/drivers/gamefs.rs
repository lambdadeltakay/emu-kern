use crate::driver::EmuRsDriverPreference;
use crate::error::EmuRsErrorReason;
use crate::vfs::EmuRsFileMetadata;
use crate::vfs::{EmuRsFileKind, EmuRsFilesystemManager};
use crate::{device::EmuRsDevice, error::EmuRsError};
use crate::{
    driver::EmuRsDriver,
    vfs::{EmuRsFsDriver, EmuRsPath},
};
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::rc::Rc;
use alloc::vec::Vec;
use tinyvec::TinyVec;

// I bet this will inflate the heap quickly
// Mounted rom database
pub struct EmuRsGameFs<'owner> {
    pub search_paths: TinyVec<[EmuRsPath<'owner>; 2]>,
    // Blake2s256 hash
    // FIXME: This is most likely extremely slow
    pub hashtable: Vec<([u8; 32], EmuRsPath<'owner>)>,
}

impl<'owner> EmuRsGameFs<'owner> {
    // FIXME: Only cache some files or something cause this will inflate ram quickly
    fn update_hashtable(&self, vfs: &mut EmuRsFilesystemManager) {
        // Get all the files in the search path
        let files = self.search_paths.iter().flat_map(|search_path| {
            return vfs
                .list_directory(&search_path)
                .unwrap()
                .into_iter()
                .filter(|filename| {
                    return vfs.metadata(&filename).unwrap().kind.unwrap() == EmuRsFileKind::File;
                });
        });
    }
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
        file: &EmuRsPath,
        buffer: &mut [u8],
        offset: usize,
    ) -> Result<(), EmuRsError> {
        return Ok(());
    }

    fn list_directory(
        &self,
        vfs: &mut EmuRsFilesystemManager,
        file: &EmuRsPath,
    ) -> Result<TinyVec<[EmuRsPath; 10]>, EmuRsError> {
        self.update_hashtable(vfs);
        return Err(EmuRsError {
            reason: EmuRsErrorReason::OperationNotSupported,
        });
    }

    fn metadata(
        &self,
        vfs: &mut EmuRsFilesystemManager,
        file: &EmuRsPath,
    ) -> Result<EmuRsFileMetadata, EmuRsError> {
        let real_file = self.hashtable.iter().find(|hash_file| {
            return hash_file.0 == file.file_name().as_bytes();
        });

        if real_file.is_none() {
            return Err(EmuRsError {
                reason: EmuRsErrorReason::InvalidPath,
            });
        }

        return Ok(vfs.metadata(&real_file.unwrap().1).unwrap());
    }
}
