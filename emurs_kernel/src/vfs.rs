use crate::error::EmuRsErrorReason;
use crate::{driver::EmuRsDriver, error::EmuRsError};
use alloc::rc::Rc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec;
use alloc::{boxed::Box, collections::BTreeMap, format};
use core::any::Any;
use core::fmt::Display;
use time::Date;
use tinyvec::{tiny_vec, TinyVec};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EmuRsFileKind {
    File,
    Folder,
    Device,
    Mount,
}

pub struct EmuRsFile {
    pub path: EmuRsPath,
    pub kind: EmuRsFileKind,
}

impl EmuRsFile {}

#[derive(Debug, Clone, PartialEq, Hash)]
/// A path with `/` seperators
pub struct EmuRsPath {
    pub segments: TinyVec<[String; 3]>,
}

impl EmuRsPath {
    pub fn is_absolute(&self) -> bool {
        return !self.segments.is_empty()
            && self.segments[0] == "/"
            && !self.segments.iter().any(|segment| {
                return *segment == ".." || *segment == ".";
            });
    }

    pub fn file_name(&self) -> String {
        return self.segments.last().cloned().unwrap();
    }
}

impl Default for EmuRsPath {
    fn default() -> Self {
        return Self {
            segments: tiny_vec!["/".to_string()],
        };
    }
}

impl Display for EmuRsPath {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.segments.join("/")).unwrap();
        return Ok(());
    }
}

/// TODO: Merge with the permissions system in mem
pub struct EmuRsPermission {
    pub read: bool,
    pub write: bool,
}

/// The VFS implementation of the operating system
///
/// Currently it will be organized like this
///
/// - `/`                               : The root of the file system
/// - `/profiles`                       : The users home directories. Note that this is not a multi user operating system and this is purely for organization
/// - `/profiles/(profile name)/saves`  : The save files for that particular profile
/// - `/system.toml`                    : The whole operating system config file
/// - `/roms`                           : The rom collection for the operating system. Contains firmware for the roms too. Roms may be selected from other locations
/// - `/roms.db`                        : The database containing blake2s hashes of known roms. Firmware has to appear here to be used but roms do not.
#[derive(Default)]
pub struct EmuRsFilesystemManager {
    mount_points: BTreeMap<EmuRsPath, Box<dyn EmuRsFsDriver>>,
}

impl EmuRsFilesystemManager {
    /// Normalize a path
    /// Wildly incomplete
    pub fn normalize_path(
        &self,
        context_path: EmuRsPath,
        relative_path: EmuRsPath,
    ) -> Result<EmuRsPath, EmuRsError> {
        let return_path = EmuRsPath::default();

        if !context_path.is_absolute() {
            return Err(EmuRsError {
                reason: EmuRsErrorReason::InvalidPath,
            });
        }

        return Ok(return_path);
    }

    pub fn read(
        &self,
        path: &EmuRsPath,
        buffer: &mut [u8],
        offset: usize,
    ) -> Result<(), EmuRsError> {
        return Err(EmuRsError {
            reason: EmuRsErrorReason::OperationNotSupported,
        });
    }

    pub fn list_directory(&self, file: &EmuRsPath) -> Result<TinyVec<[EmuRsPath; 10]>, EmuRsError> {
        return Ok(TinyVec::new());
    }

    pub fn metadata(&self, file: &EmuRsPath) -> Result<EmuRsFileMetadata, EmuRsError> {
        return Err(EmuRsError {
            reason: EmuRsErrorReason::OperationNotSupported,
        });
    }
}

/// Display the metadata of the file. Everything here is optional.
/// The misc field here is maybe not the best way to do this but who
#[derive(Debug, Clone)]
pub struct EmuRsFileMetadata {
    pub size: Option<usize>,
    pub modification_time: Option<Date>,
    pub kind: Option<EmuRsFileKind>,
}

/// The driver for a file system implementation
/// This will most likely be ustar on many, many embedded devices until I hammer out Fat or something even better
pub trait EmuRsFsDriver: EmuRsDriver {
    fn read(
        &self,
        vfs: &mut EmuRsFilesystemManager,
        file: &EmuRsPath,
        buffer: &mut [u8],
        offset: usize,
    ) -> Result<(), EmuRsError> {
        return Err(EmuRsError {
            reason: EmuRsErrorReason::OperationNotSupported,
        });
    }
    fn write(
        &self,
        vfs: &mut EmuRsFilesystemManager,
        file: &EmuRsPath,
        buffer: &[u8],
        offset: usize,
    ) -> Result<(), EmuRsError> {
        return Err(EmuRsError {
            reason: EmuRsErrorReason::OperationNotSupported,
        });
    }
    fn delete(&self, vfs: &mut EmuRsFilesystemManager, file: &EmuRsPath) -> Result<(), EmuRsError> {
        return Err(EmuRsError {
            reason: EmuRsErrorReason::OperationNotSupported,
        });
    }
    fn create(&self, vfs: &mut EmuRsFilesystemManager, file: &EmuRsPath) -> Result<(), EmuRsError> {
        return Err(EmuRsError {
            reason: EmuRsErrorReason::OperationNotSupported,
        });
    }

    fn list_directory(
        &self,
        vfs: &mut EmuRsFilesystemManager,
        file: &EmuRsPath,
    ) -> Result<TinyVec<[EmuRsPath; 10]>, EmuRsError> {
        return Err(EmuRsError {
            reason: EmuRsErrorReason::OperationNotSupported,
        });
    }

    fn metadata(
        &self,
        vfs: &mut EmuRsFilesystemManager,
        file: &EmuRsPath,
    ) -> Result<EmuRsFileMetadata, EmuRsError> {
        return Err(EmuRsError {
            reason: EmuRsErrorReason::OperationNotSupported,
        });
    }
}
