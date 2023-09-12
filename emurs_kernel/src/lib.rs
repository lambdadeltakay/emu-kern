#![no_std]
#![feature(allocator_api)]
#![allow(clippy::needless_return)]
#![feature(core_intrinsics)]

extern crate alloc;

use crate::vfs::EmuRsVfs;
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

pub fn emurs_main(
    initial_memory_table: Option<EmuRsMemoryTable>,
    mut video_driver: impl EmuRsVideoDriver,
) {
    #[cfg(feature = "embedded")]
    unsafe {
        crate::mem::EMURS_GLOBAL_MEMORY_ALLOCATOR.set_memory_table(initial_memory_table.unwrap())
    };

    video_driver.setup_hardware();
    
    video_driver.draw_polyline(
        EmuRsColorFormatRgb888::new(255, 0, 0),
        true,
        SVector::from([Point2::new(0, 0), Point2::new(10, 10)]),
    );

    let vfs = EmuRsVfs::default();
    vfs.normalize_path(
        vfs::EmuRsPath { segments: &["/"] },
        vfs::EmuRsPath { segments: &["/"] },
    )
    .unwrap();
}
