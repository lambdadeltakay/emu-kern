#![no_std]
#![feature(allocator_api)]
#![allow(clippy::needless_return)]
#![feature(core_intrinsics)]

extern crate alloc;

use crate::vfs::EmuRsVfs;
use disk::EmuRsDiskDriver;
use mem::EmuRsMemoryTable;
use nalgebra::{Point2, SVector};
use tinyvec::ArrayVec;
use video::{EmuRsColorFormatRgb888, EmuRsRgbColor, EmuRsVideoDriver};

pub mod device;
pub mod disk;
pub mod driver;
pub mod drivers;
pub mod error;
pub mod mem;
pub mod prelude;
pub mod vfs;
pub mod video;

/// The kernel entry to be used by the bootloader
///
/// Currently there is a restriction that no memory allocation may occur before the memory allocator is fed a memory table
/// Later I will add a small space of memory inside of the allocator for pre setup allocations by the bootloader
pub fn emurs_main(
    initial_memory_table: EmuRsMemoryTable,
    mut video_driver: impl EmuRsVideoDriver,
    mut disk_driver: impl EmuRsDiskDriver,
) {
    #[cfg(feature = "embedded")]
    unsafe {
        crate::mem::EMURS_GLOBAL_MEMORY_ALLOCATOR.set_memory_table(initial_memory_table)
    };

    video_driver.setup_hardware();

    for x in 0..240 {
        for y in 0..160 {
            video_driver.draw_pixel(EmuRsColorFormatRgb888::new(255, 0, 0), Point2::new(x, y));
        }
    }
}
