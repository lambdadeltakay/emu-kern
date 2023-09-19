#![no_std]
#![allow(clippy::needless_return)]
#![feature(core_intrinsics)]
#![feature(const_mut_refs)]
#![feature(allocator_api)]
#![feature(thin_box)]

extern crate alloc;

use core::borrow::BorrowMut;

use core::cell::RefCell;
use core::str::FromStr;

use crate::vfs::EmuRsFilesystemSubsystem;
use alloc::rc::Rc;
use alloc::vec::Vec;

use blake2::Digest;
use disk::EmuRsDiskDriver;
use driver::EmuRsDriver;
use drivers::gamefs::EmuRsGameFs;
use drivers::ustarfs::EmuRsUstarFs;

use nalgebra::Point2;
use prelude::EmuRsMemoryTableEntry;
use subsystem::EmuRsSubsystem;

use vfs::{EmuRsFsDriver, EmuRsPath};
use video::EmuRsGenericColor;

use video::{EmuRsRgbColor, EmuRsVideoDriver};

pub mod device;
pub mod disk;
pub mod driver;
pub mod drivers;
pub mod error;
pub mod mem;
pub mod prelude;
pub mod program;
pub mod subsystem;
pub mod vfs;
pub mod video;

#[derive(Default)]
pub struct EmuRsContextBuilder {
    pub video_drivers: Vec<Rc<RefCell<dyn EmuRsVideoDriver>>>,
    pub disk_drivers: Vec<Rc<RefCell<dyn EmuRsDiskDriver>>>,
    pub fs_drivers: Vec<Rc<RefCell<dyn EmuRsFsDriver>>>,
}

impl EmuRsContextBuilder {
    pub fn add_video_driver<DRIVER: EmuRsVideoDriver + Default + 'static>(&mut self) -> &mut Self {
        self.video_drivers
            .push(Rc::new(RefCell::new(DRIVER::default())));
        return self;
    }

    pub fn add_disk_driver<DRIVER: EmuRsDiskDriver + Default + 'static>(&mut self) -> &mut Self {
        self.disk_drivers
            .push(Rc::new(RefCell::new(DRIVER::default())));
        return self;
    }

    pub fn add_fs_driver<DRIVER: EmuRsFsDriver + Default + 'static>(&mut self) -> &mut Self {
        self.fs_drivers
            .push(Rc::new(RefCell::new(DRIVER::default())));
        return self;
    }

    pub fn done(self) -> Rc<EmuRsContext> {
        let context = Rc::new(EmuRsContext {
            fs: RefCell::new(EmuRsFilesystemSubsystem::default()),
            video_drivers: self.video_drivers,
            disk_drivers: self.disk_drivers,
            fs_drivers: self.fs_drivers,
        });

        context.fs.borrow_mut().init(context.clone());

        for driver in context.video_drivers.iter() {
            driver.as_ref().borrow_mut().init(context.clone());
        }

        for driver in context.disk_drivers.iter() {
            driver.as_ref().borrow_mut().init(context.clone());
        }

        for driver in context.fs_drivers.iter() {
            driver.as_ref().borrow_mut().init(context.clone());
        }

        return context;
    }
}

#[derive(Clone)]
pub struct EmuRsContext {
    pub fs: RefCell<EmuRsFilesystemSubsystem>,
    pub video_drivers: Vec<Rc<RefCell<dyn EmuRsVideoDriver>>>,
    pub disk_drivers: Vec<Rc<RefCell<dyn EmuRsDiskDriver>>>,
    pub fs_drivers: Vec<Rc<RefCell<dyn EmuRsFsDriver>>>,
}

/// The kernel entry to be used by the bootloader
///
/// Currently there is a restriction that no memory allocation may occur before the memory allocator is fed a memory table
/// Later I will add a small space of memory inside of the allocator for pre setup allocations by the bootloader
pub fn emurs_main(
    memory_table_entries: &[EmuRsMemoryTableEntry],
    driver_setup_callback: fn(&mut EmuRsContextBuilder),
) -> ! {
    unsafe {
        crate::mem::EMURS_GLOBAL_MEMORY_ALLOCATOR.add_memory_table_entries(memory_table_entries)
    };

    // We implement a callback so the drivers can use alloc if it would please them
    let mut builder = EmuRsContextBuilder::default();
    driver_setup_callback(&mut builder);

    // Add some fs drivers
    builder
        .add_fs_driver::<EmuRsGameFs>()
        .add_fs_driver::<EmuRsUstarFs>();

    let context = builder.done();

    for x in 0..100 {
        for y in 0..100 {
            context.video_drivers[0]
                .as_ref()
                .borrow_mut()
                .draw_pixel(EmuRsGenericColor::new(0xff, 0x00, 0x00), Point2::new(x, y));
        }
    }

    let files = context
        .fs
        .borrow()
        .list_directory(&EmuRsPath::from_str("/").unwrap())
        .unwrap();

    loop {}
}
