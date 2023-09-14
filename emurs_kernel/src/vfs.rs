use crate::error::EmuRsErrorReason;
use crate::{driver::EmuRsDriver, error::EmuRsError};
use alloc::vec;
use alloc::{boxed::Box, collections::BTreeMap, format};
use core::any::Any;
use core::fmt::Display;
use time::Date;
use tinyvec::{tiny_vec, TinyVec};

#[derive(Debug)]
pub enum EmuRsFileKind {
    File,
    Folder,
    Device,
    Mount,
}

pub struct EmuRsFile<'owner> {
    pub path: EmuRsPath<'owner>,
    pub kind: EmuRsFileKind,
}

impl<'owner> EmuRsFile<'owner> {}

/// A path with `/` seperators
pub struct EmuRsPath<'owner> {
    pub segments: TinyVec<[&'owner str; 3]>,
}

impl<'owner> EmuRsPath<'owner> {
    pub fn is_absolute(&self) -> bool {
        return !self.segments.is_empty()
            && self.segments[0] == "/"
            && !self.segments.iter().any(|segment| {
                return *segment == ".." || *segment == ".";
            });
    }
}

impl<'owner> Default for EmuRsPath<'owner> {
    fn default() -> Self {
        return Self {
            segments: tiny_vec!["/"],
        };
    }
}

impl<'owner> Display for EmuRsPath<'owner> {
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
/// - `/roms.cache`                     : Cache of the hashes of the rom collection (may remove)
/// - `/roms.db`                        : The database containing blake2b hashes of known roms. Firmware has to appear here to be used but roms do not.
#[derive(Default)]
pub struct EmuRsVfs<'owner> {
    mount_points: BTreeMap<EmuRsPath<'owner>, Box<dyn EmuRsFsDriver>>,
}

impl<'owner> EmuRsVfs<'owner> {
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
}

#[derive(Debug)]
pub struct EmuRsFileMetadata<'owner> {
    pub modification_time: Option<Date>,
    pub kind: Option<EmuRsFileKind>,
    pub misc: Option<BTreeMap<&'owner str, &'owner dyn Any>>,
}

/// The driver for a file system implementation
/// This will most likely be ustar on many, many embedded devices until I hammer out Fat or something even better
pub trait EmuRsFsDriver: EmuRsDriver {
    fn read(
        &self,
        vfs: &mut EmuRsVfs,
        file: EmuRsPath,
        buffer: &[u8],
        offset: usize,
    ) -> Result<(), EmuRsError> {
        return Err(EmuRsError {
            reason: EmuRsErrorReason::OperationNotSupported,
        });
    }
    fn write(
        &self,
        vfs: &mut EmuRsVfs,
        file: EmuRsPath,
        buffer: &mut [u8],
        offset: usize,
    ) -> Result<(), EmuRsError> {
        return Err(EmuRsError {
            reason: EmuRsErrorReason::OperationNotSupported,
        });
    }
    fn delete(&self, vfs: &mut EmuRsVfs, file: EmuRsPath) -> Result<(), EmuRsError> {
        return Err(EmuRsError {
            reason: EmuRsErrorReason::OperationNotSupported,
        });
    }
    fn create(&self, vfs: &mut EmuRsVfs, file: EmuRsPath) -> Result<(), EmuRsError> {
        return Err(EmuRsError {
            reason: EmuRsErrorReason::OperationNotSupported,
        });
    }

    fn list_directory(&self, vfs: &mut EmuRsVfs, file: EmuRsPath) -> Result<(), EmuRsError> {
        return Err(EmuRsError {
            reason: EmuRsErrorReason::OperationNotSupported,
        });
    }

    fn metadata(&self, vfs: &mut EmuRsVfs, file: EmuRsPath) -> Result<(), EmuRsError> {
        return Err(EmuRsError {
            reason: EmuRsErrorReason::OperationNotSupported,
        });
    }
}
