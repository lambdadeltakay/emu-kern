#![no_std]
#![allow(clippy::needless_return)]
#![feature(core_intrinsics)]
#![feature(const_mut_refs)]
#![feature(allocator_api)]

extern crate alloc;

use crate::vfs::EmuRsFilesystemManager;
use alloc::vec::Vec;
use blake2::Blake2s256;
use blake2::Digest;
use disk::EmuRsDiskDriver;
use mem::EmuRsMemoryTable;
use nalgebra::{Point2, SVector};
use prelude::EmuRsMemoryTableEntry;
use tinyvec::ArrayVec;
use video::EmuRsGenericColor;
use video::EmuRsTexture;
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
    pub fs: EmuRsFilesystemManager,
    video_drivers: &'owner mut [&'owner mut dyn EmuRsVideoDriver],
    disk_drivers: &'owner mut [&'owner mut dyn EmuRsDiskDriver],
}

/// The kernel entry to be used by the bootloader
///
/// Currently there is a restriction that no memory allocation may occur before the memory allocator is fed a memory table
/// Later I will add a small space of memory inside of the allocator for pre setup allocations by the bootloader
pub fn emurs_main(
    memory_table_entries: &[EmuRsMemoryTableEntry],
    mut video_driver: &mut [&mut dyn EmuRsVideoDriver],
    mut disk_driver: &mut [&mut dyn EmuRsDiskDriver],
) -> ! {
    unsafe {
        crate::mem::EMURS_GLOBAL_MEMORY_ALLOCATOR.add_memory_table_entries(memory_table_entries)
    };

    video_driver[0].setup_hardware();

    loop {
        let random_bytes = include_bytes!("mem.rs");
        video_driver[0].draw_texture(
            EmuRsTexture::new(
                &random_bytes
                    .into_iter()
                    .map(|byte| {
                        return EmuRsGenericColor::new(*byte, *byte, *byte);
                    })
                    .collect::<Vec<_>>(),
                Point2::new(100, 100),
            ),
            Point2::new(0, 0),
        );
    }
}
