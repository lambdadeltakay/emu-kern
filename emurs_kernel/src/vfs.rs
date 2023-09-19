use crate::error::EmuRsErrorReason;
use crate::subsystem::EmuRsSubsystem;
use crate::EmuRsContext;
use crate::{driver::EmuRsDriver, error::EmuRsError};
use alloc::collections::BTreeMap;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec;
use core::fmt::Display;
use core::str::FromStr;
use time::Date;
use tinyvec::{tiny_vec, TinyVec};

/// The VFS implementation of the operating system
///
/// Currently it will be organized like this
///
/// - `ROOT/`                               : The root of the file system
/// - `ROOT/profiles`                       : The users home directories. Note that this is not a multi user operating system and this is purely for organization
/// - `ROOT/profiles/(profile name)/saves`  : The save files for that particular profile
/// - `ROOT/system.toml`                    : The whole operating system config file
/// - `ROOT/roms`                           : The rom collection for the operating system. Contains firmware for the roms too. Roms may be selected from other locations
/// - `ROOT/roms.db`                        : The database containing blake2s hashes of known roms. Firmware has to appear here to be used but roms do not.
#[derive(Clone, Default)]
pub struct EmuRsFilesystemSubsystem {
    os_context: Option<Rc<EmuRsContext>>,
    fsdriver_to_diskdriver: BTreeMap<&'static str, &'static str>,
    mountpounts: BTreeMap<EmuRsPath, &'static str>,
}

impl EmuRsFilesystemSubsystem {
    /// Normalize a path
    /// Wildly incomplete
    pub fn normalize_path(
        &self,
        context_path: EmuRsPath,
        _relative_path: EmuRsPath,
    ) -> Result<EmuRsPath, EmuRsError> {
        let return_path = EmuRsPath::default();

        if !context_path.is_absolute() {
            return Err(EmuRsError {
                reason: EmuRsErrorReason::InvalidPath,
            });
        }

        return Ok(return_path);
    }

    pub fn is_child_of(
        &self,
        potential_parent: &EmuRsPath,
        potential_child: &EmuRsPath,
    ) -> Result<bool, EmuRsError> {
        if !potential_parent.is_absolute() {
            return Err(EmuRsError {
                reason: EmuRsErrorReason::InvalidPath,
            });
        }
    }

    pub fn read(
        &self,
        _path: &EmuRsPath,
        _buffer: &mut [u8],
        _offset: usize,
    ) -> Result<(), EmuRsError> {
        return Err(EmuRsError {
            reason: EmuRsErrorReason::OperationNotSupported,
        });
    }

    pub fn list_directory(&self, file: &EmuRsPath) -> Result<TinyVec<[EmuRsPath; 10]>, EmuRsError> {
        return Ok(TinyVec::new());
    }

    pub fn metadata(&self, _file: &EmuRsPath) -> Result<EmuRsFileMetadata, EmuRsError> {
        return Err(EmuRsError {
            reason: EmuRsErrorReason::OperationNotSupported,
        });
    }
}

impl EmuRsSubsystem for EmuRsFilesystemSubsystem {
    fn init(&mut self, context: Rc<EmuRsContext>) {
        self.os_context = Some(context);
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
        &mut self,
        _file: &EmuRsPath,
        _buffer: &mut [u8],
        _offset: usize,
    ) -> Result<(), EmuRsError> {
        return Err(EmuRsError {
            reason: EmuRsErrorReason::OperationNotSupported,
        });
    }
    fn write(
        &mut self,
        _file: &EmuRsPath,
        _buffer: &[u8],
        _offset: usize,
    ) -> Result<(), EmuRsError> {
        return Err(EmuRsError {
            reason: EmuRsErrorReason::OperationNotSupported,
        });
    }
    fn delete(&mut self, _file: &EmuRsPath) -> Result<(), EmuRsError> {
        return Err(EmuRsError {
            reason: EmuRsErrorReason::OperationNotSupported,
        });
    }
    fn create(&mut self, _file: &EmuRsPath) -> Result<(), EmuRsError> {
        return Err(EmuRsError {
            reason: EmuRsErrorReason::OperationNotSupported,
        });
    }

    fn list_directory(
        &mut self,
        _file: &EmuRsPath,
    ) -> Result<TinyVec<[EmuRsPath; 10]>, EmuRsError> {
        return Err(EmuRsError {
            reason: EmuRsErrorReason::OperationNotSupported,
        });
    }

    fn metadata(&mut self, _file: &EmuRsPath) -> Result<EmuRsFileMetadata, EmuRsError> {
        return Err(EmuRsError {
            reason: EmuRsErrorReason::OperationNotSupported,
        });
    }
}

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
        return self.segments[0] == "ROOT"
            || (self.is_root()
                && !self.segments.iter().any(|segment| {
                    return *segment == ".." || *segment == ".";
                }));
    }

    pub fn is_root(&self) -> bool {
        return self.segments[0] == "ROOT";
    }

    pub fn file_name(&self) -> String {
        return self.segments.last().cloned().unwrap();
    }

    pub fn is_valid(&self) -> bool {
        for segment in self.segments.iter() {
            if segment == "/" {
                return false;
            }
        }

        return true;
    }
}

impl FromStr for EmuRsPath {
    type Err = EmuRsError;

    // TODO: Complete this
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s[0..1] == "fs" {
            return Ok(Self {
                segments: tiny_vec![s.to_string()],
            });
        }

        return Err(EmuRsError {
            reason: EmuRsErrorReason::InvalidPath,
        });
    }
}

impl Default for EmuRsPath {
    fn default() -> Self {
        return Self {
            segments: tiny_vec!["fs".to_string()],
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
