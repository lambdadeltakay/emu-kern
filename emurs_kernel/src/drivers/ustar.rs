use alloc::boxed::Box;
use crate::{disk::EmuRsDiskDriver, vfs::{EmuRsVfs, EmuRsPath}};

#[inline]
fn ustar_lookup(disk: &mut EmuRsVfs, disk_filename: EmuRsPath, target_filename: EmuRsPath)
{
    
}