use crate::driver::EmuRsDriverPreference;
use crate::error::EmuRsErrorReason;
use crate::vfs::EmuRsFileMetadata;
use crate::vfs::{EmuRsFileKind};
use crate::EmuRsContext;
use crate::{device::EmuRsDevice, error::EmuRsError};
use crate::{
    driver::EmuRsDriver,
    vfs::{EmuRsFsDriver, EmuRsPath},
};

use alloc::collections::{BTreeMap};

use alloc::rc::Rc;
use alloc::string::String;
use alloc::vec::Vec;
use blake2::Blake2s256;
use blake2::Digest;

use core::fmt::Write;
use core::str::FromStr;
use tinyvec::TinyVec;

// I bet this will inflate the heap quickly
// Mounted rom database

#[derive(Default)]
pub struct EmuRsGameFs {
    pub search_paths: Vec<EmuRsPath>,
    pub os_context: Option<Rc<EmuRsContext>>,
}

impl EmuRsGameFs {
    /// Hash the files in the search directories and return our findings
    fn get_hashtable(&self) -> BTreeMap<[u8; 32], EmuRsPath> {
        let mut files = BTreeMap::new();
        for path in self.search_paths.iter() {
            let directory_contents = self
                .os_context
                .as_ref()
                .unwrap()
                .fs
                .borrow_mut()
                .list_directory(&path)
                .unwrap();

            files.extend(
                directory_contents
                    .iter()
                    .filter(|filename| {
                        return self
                            .os_context
                            .as_ref()
                            .unwrap()
                            .fs
                            .borrow()
                            .metadata(&filename)
                            .unwrap()
                            .kind
                            .unwrap()
                            == EmuRsFileKind::File;
                    })
                    .map(|file| {
                        let file_len = self
                            .os_context
                            .as_ref()
                            .unwrap()
                            .fs
                            .borrow()
                            .metadata(&file)
                            .unwrap()
                            .size
                            .unwrap();

                        // FIXME: REALLY GOOD WAY TO FILL RAM
                        let buffer = Vec::with_capacity(file_len);
                        let mut hasher = Blake2s256::new();
                        hasher.update(&buffer);
                        return (hasher.finalize()[..].try_into().unwrap(), file.clone());
                    }),
            );
        }
        return files;
    }
}

impl EmuRsDriver for EmuRsGameFs {
    fn name(&self) -> &'static str {
        return "Game Filesystem";
    }

    fn get_preference(&mut self) -> EmuRsDriverPreference {
        todo!()
    }

    fn get_claimed(&mut self) -> EmuRsDevice {
        todo!()
    }

    fn init(&mut self, context: Rc<EmuRsContext>) {
        self.os_context = Some(context);
    }
}

impl EmuRsFsDriver for EmuRsGameFs {
    fn read(
        &mut self,
        _file: &EmuRsPath,
        _buffer: &mut [u8],
        _offset: usize,
    ) -> Result<(), EmuRsError> {
        return Err(EmuRsError {
            reason: EmuRsErrorReason::OperationNotSupported,
        });
    }

    fn list_directory(&mut self, _file: &EmuRsPath) -> Result<TinyVec<[EmuRsPath; 10]>, EmuRsError> {
        return Ok(self
            .get_hashtable()
            .into_keys()
            .map(|key| {
                let mut string = String::new();
                for byte in key {
                    write!(string, "{:x}", byte);
                }
                return EmuRsPath::from_str(&string).unwrap();
            })
            .collect());
    }

    fn metadata(&mut self, _file: &EmuRsPath) -> Result<EmuRsFileMetadata, EmuRsError> {
        return Err(EmuRsError {
            reason: EmuRsErrorReason::OperationNotSupported,
        });
    }
}
