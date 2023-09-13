#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(allocator_api)]

extern crate alloc;

mod video;

use core::ptr::NonNull;

use emurs_kernel::device::EmuRsDevice;
use emurs_kernel::disk::{EmuRsDiskDriver, EmuRsDummyDiskDriver, EmuRsMemoryDisk};
use emurs_kernel::driver::{EmuRsDriver, EmuRsDriverPreference};
use emurs_kernel::prelude::tinyvec::{array_vec, TinyVec};
use emurs_kernel::{mem::EmuRsMemoryRange, prelude::*};
use video::GbaVideo;

#[naked]
#[no_mangle]
#[instruction_set(arm::a32)]
#[link_section = ".text.gba_rom_header"]
unsafe extern "C" fn _start() -> ! {
    core::arch::asm!(
        // Jump and leave memory for the header
        "b 1f",
        ".space 188",
        "1:",
        // Change to system mode
        "mov r0, #0x1f",
        "msr CPSR_c, r0",
        // Setup the stack
        "ldr sp, =0x3007F00",
        // Jump to our main function and switch to thumb
        "ldr r2, =gba_loader",
        "bx r2",
        // Restart upon return
        "swi #0",
        options(noreturn)
    )
}

// For now just setup external work ram as a heap
#[no_mangle]
pub extern "C" fn gba_loader() -> ! {
    emurs_main(
        &[EmuRsMemoryTableEntry {
            permissions: EmuRsMemoryPermission {
                read: true,
                write: true,
                execute: true,
            },
            range: EmuRsMemoryRange::new(0x2000000, 0x203ffff),
            kind: EmuRsMemoryKind::Work,
        }],
        &mut [&mut GbaVideo],
        &mut [&mut GbaSram],
    );

    loop {}
}

pub struct GbaSram;

impl EmuRsDriver for GbaSram {
    fn name(&self) -> &str {
        return "Game Boy Advance SRAM";
    }

    fn get_claimed(&self) -> EmuRsDevice {
        todo!()
    }

    fn setup_hardware(&self) {}

    fn get_preference(&self) -> EmuRsDriverPreference {
        return EmuRsDriverPreference::Preferred;
    }
}

impl EmuRsMemoryDisk for GbaSram {
    fn get_memory(&self) -> &mut [u8] {
        return unsafe { core::slice::from_raw_parts_mut(0xe000000 as *mut u8, 0xffff) };
    }
}
