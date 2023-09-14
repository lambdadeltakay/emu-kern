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
use blake2::Blake2s256;
use blake2::Digest;
use core::cell::RefCell;
use tinyvec::TinyVec;

// I bet this will inflate the heap quickly
// Mounted rom database
pub struct EmuRsGameFs {
    pub search_paths: TinyVec<[EmuRsPath; 2]>,
    // Blake2s256 hash
    // FIXME: This is most likely extremely slow
    pub hashtable: RefCell<Vec<([u8; 32], EmuRsPath)>>,
}

impl EmuRsGameFs {
    // FIXME: Only cache some files or something cause this will inflate ram quickly
    fn update_hashtable(&self, vfs: &mut EmuRsFilesystemManager) {
        // Get all the files in the search path

        for path in self.search_paths.iter() {
            vfs.list_directory(&path)
                .clone()
                .unwrap()
                .iter()
                .filter(|filename| {
                    return vfs.metadata(&filename).unwrap().kind.unwrap() == EmuRsFileKind::File;
                })
                .for_each(|file| {
                    let file_len = vfs.metadata(&file).unwrap().size.unwrap();

                    // FIXME: REALLY GOOD WAY TO FILL RAM
                    let buffer = Vec::with_capacity(file_len);
                    let mut hasher = Blake2s256::new();
                    hasher.update(&buffer);
                    let hash = &hasher.finalize()[..];

                    let found = self.hashtable.borrow().iter().position(|block| {
                        return block.0 == hash;
                    });

                    if found.is_some() {
                        self.hashtable.borrow_mut().remove(found.unwrap());
                    }

                    self.hashtable
                        .borrow_mut()
                        .push((hash.clone().try_into().unwrap(), file.clone()));
                });
        }
    }
}

impl EmuRsDriver for EmuRsGameFs {
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

impl EmuRsFsDriver for EmuRsGameFs {
    fn read(
        &self,
        vfs: &mut EmuRsFilesystemManager,
        file: &EmuRsPath,
        mut buffer: &mut [u8],
        offset: usize,
    ) -> Result<(), EmuRsError> {
        self.update_hashtable(vfs);

        let hashtable = self.hashtable.borrow();

        let real_file = hashtable.iter().find(|hash_file| {
            return hash_file.0 == file.file_name().as_bytes();
        });

        if real_file.is_none() {
            return Err(EmuRsError {
                reason: EmuRsErrorReason::InvalidPath,
            });
        }

        let res = vfs.read(&real_file.unwrap().1, &mut buffer, offset);

        return Ok(res.unwrap());
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
        self.update_hashtable(vfs);

        let real_file = self
            .hashtable
            .borrow()
            .iter()
            .find(|hash_file| {
                return hash_file.0 == file.file_name().as_bytes();
            })
            .cloned();

        if real_file.is_none() {
            return Err(EmuRsError {
                reason: EmuRsErrorReason::InvalidPath,
            });
        }

        return Ok(vfs.metadata(&real_file.unwrap().1).unwrap());
    }
}
