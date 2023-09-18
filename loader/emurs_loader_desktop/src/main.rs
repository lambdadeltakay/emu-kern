#![feature(test)]
extern crate test;

#[allow(unused_imports)]
use emurs_kernel::prelude::*;
use emurs_kernel::video::EmuRsColorFormatRgb565;
use emurs_kernel::{
    mem::EmuRsMemoryRange,
    prelude::tinyvec::{array_vec, ArrayVec},
    video::{
        EmuRsColor, EmuRsColorFormatRgb111, EmuRsColorFormatRgb222, EmuRsColorFormatRgb333,
        EmuRsColorFormatRgb444, EmuRsColorFormatRgb555, EmuRsColorFormatRgb666,
        EmuRsColorFormatRgb777, EmuRsColorFormatRgb888, EmuRsRgbColor,
    },
};

// For now just setup external work ram as a heap
pub fn main() {
    let mut buffer = [0_u8; 1000];

    emurs_main(
        &[EmuRsMemoryTableEntry {
            permissions: EmuRsMemoryPermission {
                read: true,
                write: true,
                execute: false,
            },
            range: EmuRsMemoryRange::new(
                buffer.as_mut_ptr() as usize,
                buffer.as_mut_ptr() as usize + buffer.len(),
            ),
            kind: EmuRsMemoryKind::Work,
        }],
        |context| {},
    );
}

use test::Bencher;

#[test]
fn test_color_conversion() {
    (0..u8::MAX).for_each(|num| {
        let color = EmuRsColorFormatRgb888::new(num, num, num)
            .convert_rgb::<EmuRsColorFormatRgb777>()
            .convert_rgb::<EmuRsColorFormatRgb666>()
            .convert_rgb::<EmuRsColorFormatRgb565>()
            .convert_rgb::<EmuRsColorFormatRgb555>()
            .convert_rgb::<EmuRsColorFormatRgb444>()
            .convert_rgb::<EmuRsColorFormatRgb333>()
            .convert_rgb::<EmuRsColorFormatRgb222>()
            .convert_rgb::<EmuRsColorFormatRgb111>();

        println!("{}: {:?}", num, color);
    });
}
