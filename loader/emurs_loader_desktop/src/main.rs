#![feature(test)]
extern crate test;

#[allow(unused_imports)]
use emurs_kernel::prelude::*;
use emurs_kernel::{
    mem::EmuRsMemoryRange,
    prelude::tinyvec::{array_vec, ArrayVec},
    video::{
        EmuRsColorFormatRgb111, EmuRsColorFormatRgb222, EmuRsColorFormatRgb333,
        EmuRsColorFormatRgb444, EmuRsColorFormatRgb555, EmuRsColorFormatRgb666,
        EmuRsColorFormatRgb777, EmuRsColorFormatRgb888, EmuRsRgbColor, EmuRsDummyVideoDriver,
    },
};

// For now just setup external work ram as a heap
pub fn main() {
    emurs_main(None, EmuRsDummyVideoDriver);
}

use test::Bencher;

#[bench]
fn test_color_conversion(b: &mut Bencher) {
    b.iter(|| {
        (0..u8::MAX).for_each(|num| {
            EmuRsColorFormatRgb111::from(EmuRsColorFormatRgb222::from(
                EmuRsColorFormatRgb333::from(EmuRsColorFormatRgb444::from(
                    EmuRsColorFormatRgb555::from(EmuRsColorFormatRgb666::from(
                        EmuRsColorFormatRgb777::from(EmuRsColorFormatRgb888::new(num, num, num)),
                    )),
                )),
            ));
        });
    });
}
