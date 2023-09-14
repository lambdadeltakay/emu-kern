#![no_std]
#![feature(allocator_api)]
#![allow(clippy::needless_return)]
#![feature(core_intrinsics)]
#![feature(type_alias_impl_trait)]

extern crate alloc;

use crate::vfs::EmuRsVfs;
use alloc::vec::Vec;
use disk::EmuRsDiskDriver;
use mem::EmuRsMemoryTable;
use nalgebra::{Point2, SVector};
use prelude::EmuRsMemoryTableEntry;
use tinyvec::ArrayVec;
use video::EmuRsGenericColor;
use video::{EmuRsColorFormatRgb888, EmuRsRgbColor, EmuRsVideoDriver};

pub mod device;
pub mod disk;
pub mod driver;
pub mod drivers;
pub mod error;
pub mod mem;
pub mod prelude;
pub mod program;
pub mod vfs;
pub mod video;


pub struct EmuRsContext<'owner> {
    pub fs: EmuRsVfs<'owner>,
    video_drivers: &'owner mut [&'owner mut dyn EmuRsVideoDriver],
}

/// The kernel entry to be used by the bootloader
///
/// Currently there is a restriction that no memory allocation may occur before the memory allocator is fed a memory table
/// Later I will add a small space of memory inside of the allocator for pre setup allocations by the bootloader
pub fn emurs_main(
    memory_table_entries: &[EmuRsMemoryTableEntry],
    mut video_driver: &mut [&mut dyn EmuRsVideoDriver],
    mut disk_driver: &mut [&mut dyn EmuRsDiskDriver],
) {
    #[cfg(feature = "embedded")]
    unsafe {
        crate::mem::EMURS_GLOBAL_MEMORY_ALLOCATOR.add_memory_table_entries(memory_table_entries)
    };

    video_driver[0].setup_hardware();

    // Silly little test to stress what we have so far
    for x in 0..240 {
        for y in 0..160 {
            video_driver[0].draw_pixel(EmuRsGenericColor::new(0, 0xff, 0xff), Point2::new(x, y));
        }
    }
}
