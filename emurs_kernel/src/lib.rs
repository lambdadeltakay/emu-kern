#![no_std]
#![feature(allocator_api)]
#![allow(clippy::needless_return)]
#![feature(core_intrinsics)]

extern crate alloc;

use crate::vfs::EmuRsVfs;
use alloc::vec::Vec;
use blake2::Blake2b512;
use blake2::Digest;
use disk::EmuRsDiskDriver;
use mem::EmuRsMemoryTable;
use nalgebra::{Point2, SVector};
use prelude::EmuRsMemoryTableEntry;
use tinyvec::ArrayVec;
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

/// The kernel entry to be used by the bootloader
///
/// Currently there is a restriction that no memory allocation may occur before the memory allocator is fed a memory table
/// Later I will add a small space of memory inside of the allocator for pre setup allocations by the bootloader
pub fn emurs_main(
    memory_table_entries: &[EmuRsMemoryTableEntry],
    mut video_driver: impl EmuRsVideoDriver,
    mut disk_driver: impl EmuRsDiskDriver,
) {
    #[cfg(feature = "embedded")]
    unsafe {
        crate::mem::EMURS_GLOBAL_MEMORY_ALLOCATOR.add_memory_table_entries(memory_table_entries)
    };

    video_driver.setup_hardware();

    // Silly little test to stress what we have so far
    for x in 0..240 {
        for y in 0..160 {
            let mut buffer = Vec::with_capacity(100);
            disk_driver.read(&mut buffer, x as usize + y as usize);

            let mut hasher = Blake2b512::new();
            hasher.update(buffer);
            let hash = hasher.finalize();

            video_driver.draw_polyline(
                &hash
                    .iter()
                    .map(|block| {
                        return Point2::new(
                            (*block.min(&240)) as usize,
                            (*block.min(&160)) as usize,
                        );
                    })
                    .collect::<Vec<_>>(),
                EmuRsColorFormatRgb888::new(0xff, x, y),
                true,
            );

            disk_driver.write(&[x as u8, y as u8], x as usize);
        }
    }
}
