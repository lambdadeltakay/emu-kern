#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(allocator_api)]
#![feature(const_mut_refs)]

extern crate alloc;

mod video;

use emurs_kernel::prelude::tinyvec::{array_vec, TinyVec};
use emurs_kernel::{mem::EmuRsMemoryRange, prelude::*};
use video::GbaVideo;
use emurs_kernel::disk::EmuRsDummyDiskDriver;

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
        EmuRsMemoryTable {
            entries: array_vec![EmuRsMemoryTableEntry {
                permissions: EmuRsMemoryPermission {
                    read: true,
                    write: true,
                    execute: true,
                },
                range: EmuRsMemoryRange::new(0x2000000, 0x203ffff),
                kind: EmuRsMemoryKind::Work
            }],
        },
        GbaVideo, EmuRsDummyDiskDriver
    );

    loop {}
}
