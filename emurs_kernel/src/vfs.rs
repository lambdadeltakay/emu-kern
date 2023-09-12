use crate::{driver::EmuRsDriver, error::EmuRsError};
use alloc::{boxed::Box, collections::BTreeMap, format};
use core::fmt::Display;

pub struct EmuRsPath<'owner> {
    pub segments: &'owner [&'owner str],
}

impl<'owner> EmuRsPath<'owner> {
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

pub struct EmuRsPermission {
    pub read: bool,
    pub write: bool,
}

#[derive(Default)]
pub struct EmuRsVfs<'owner> {
    mount_points: BTreeMap<EmuRsPath<'owner>, Box<dyn EmuRsFsBackEnd>>,
}

impl<'owner> EmuRsVfs<'owner> {
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

        if result.is_some() {
            return Err(EmuRsError {
                message: format!("Path {} is not absolute", context_path),
            });
        }

        let new_path = EmuRsPath::default();

        return Ok(new_path);
    }
}

pub trait EmuRsFsBackEnd: EmuRsDriver {
    fn read(&self, file: EmuRsPath, buffer: &[u8], offset: usize);
    fn write(&self, file: EmuRsPath, buffer: &mut [u8], offset: usize);
    fn delete(&self, file: EmuRsPath);
    fn create(&self, file: EmuRsPath);
}
