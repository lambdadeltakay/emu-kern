#![no_std]
#![allow(clippy::needless_return)]
#![feature(core_intrinsics)]
#![feature(const_mut_refs)]
#![feature(allocator_api)]
#![feature(thin_box)]

extern crate alloc;

use core::cell::RefCell;
use core::marker::PhantomData;

use crate::vfs::EmuRsFilesystemManager;
use alloc::boxed::Box;
use alloc::boxed::ThinBox;
use alloc::rc::Rc;
use alloc::vec::Vec;
use blake2::Blake2s256;
use blake2::Digest;
use disk::EmuRsDiskDriver;
use lock_api::Mutex;
use mem::EmuRsMemoryTable;
use nalgebra::{Point2, SVector};
use prelude::EmuRsMemoryTableEntry;
use tinyvec::ArrayVec;
use tinyvec::TinyVec;
use vfs::EmuRsFsDriver;
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

#[derive(Default)]
pub struct EmuRsContext {
    pub fs: EmuRsFilesystemManager,
    video_drivers: Vec<Rc<RefCell<dyn EmuRsVideoDriver>>>,
    disk_drivers: Vec<Rc<RefCell<dyn EmuRsDiskDriver>>>,
    fs_drivers: Vec<Rc<RefCell<dyn EmuRsFsDriver>>>,
}

impl EmuRsContext {
    pub fn new() -> Rc<RefCell<Self>> {
        return Rc::new(RefCell::new(Self::default()));
    }

    pub fn add_video_driver<DRIVER: EmuRsVideoDriver + Default + 'static>(&mut self) {
        self.video_drivers
            .push(Rc::new(RefCell::new(DRIVER::default())));
    }

    pub fn add_disk_driver<DRIVER: EmuRsDiskDriver + Default + 'static>(&mut self) {
        self.disk_drivers
            .push(Rc::new(RefCell::new(DRIVER::default())));
    }

    pub fn add_fs_driver<DRIVER: EmuRsFsDriver + Default + 'static>(&mut self) {
        self.fs_drivers
            .push(Rc::new(RefCell::new(DRIVER::default())));
    }
}

/// The kernel entry to be used by the bootloader
///
/// Currently there is a restriction that no memory allocation may occur before the memory allocator is fed a memory table
/// Later I will add a small space of memory inside of the allocator for pre setup allocations by the bootloader
pub fn emurs_main(
    memory_table_entries: &[EmuRsMemoryTableEntry],
    driver_setup_callback: fn(Rc<RefCell<EmuRsContext>>),
) -> ! {
    unsafe {
        crate::mem::EMURS_GLOBAL_MEMORY_ALLOCATOR.add_memory_table_entries(memory_table_entries)
    };

    let context = EmuRsContext::new();
    driver_setup_callback(context);

    loop {}
    // video_driver[0].setup_hardware();
}
