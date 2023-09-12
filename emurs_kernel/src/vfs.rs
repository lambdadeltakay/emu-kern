use crate::{driver::EmuRsDriver, error::EmuRsError};
use alloc::{boxed::Box, collections::BTreeMap, format};
use core::fmt::Display;

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
    pub segments: &'owner [&'owner str],
}

impl<'owner> EmuRsPath<'owner> {
    /// Does a partial check if the path is valid
    /// A full check could only be done by [EmuRsVfs]
    pub fn is_valid(&self) -> bool {
        return self.segments[0] == "/" || self.segments[0] == ".." || self.segments[0] == ".";
    }
}

impl<'owner> Default for EmuRsPath<'owner> {
    fn default() -> Self {
        return Self { segments: &["/"] };
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
    mount_points: BTreeMap<EmuRsPath<'owner>, Box<dyn EmuRsFsBackEnd>>,
}

impl<'owner> EmuRsVfs<'owner> {
    /// Normalize a path
    /// Wildly incomplete
    pub fn normalize_path(
        &self,
        mut context_path: EmuRsPath,
        relative_path: EmuRsPath,
    ) -> Result<EmuRsPath, EmuRsError> {
        if context_path.segments.is_empty() {
            context_path.segments = &["/"];
        }

        let result = context_path.segments.iter().find(|segment| {
            return **segment == ".." || **segment == ".";
        });

        if result.is_some() {}

        todo!();

        let new_path = EmuRsPath::default();

        return Ok(new_path);
    }
}

/// The driver for a file system implementation
/// This will most likely be ustar on many, many embedded devices until I hammer out Fat or something even better
pub trait EmuRsFsBackEnd: EmuRsDriver {
    fn read(&self, file: EmuRsPath, buffer: &[u8], offset: usize);
    fn write(&self, file: EmuRsPath, buffer: &mut [u8], offset: usize);

    fn delete(&self, file: EmuRsPath);
    fn create(&self, file: EmuRsPath);
}
