use alloc::string::String;
use emurs_kernel::device::EmuRsDevice;
use emurs_kernel::driver::*;
use emurs_kernel::prelude::bitfield::bitfield;
use emurs_kernel::prelude::lock_api::{Mutex, RawMutex};
use emurs_kernel::prelude::spin::mutex::spin;
use emurs_kernel::prelude::tinyvec::{array_vec, ArrayVec};
use emurs_kernel::video::{EmuRsColor, EmuRsColorFormatRgb565, EmuRsRgbColor};
use emurs_kernel::{
    prelude::nalgebra::{Point2, Vector2},
    video::EmuRsVideoDriver,
};

bitfield! {
    pub struct DisplayControl(u8);
    pub background_mode, set_background_mode: 3, 0;
}

const DISPCNT: *mut DisplayControl = 0x4000000 as *mut DisplayControl;

#[derive(Default)]
pub struct GbaVideo;

impl EmuRsDriver for GbaVideo {
    fn name(&self) -> &str {
        return "Game Boy Advance Video Driver";
    }
    fn get_claimed(&self) -> EmuRsDevice {
        todo!()
    }
    fn setup_hardware(&self) {
        unsafe { DISPCNT.as_mut().unwrap().set_background_mode(3) };
    }
}

impl EmuRsVideoDriver for GbaVideo {
    fn draw_pixel(&mut self, color: impl EmuRsRgbColor, position: Point2<usize>) {
        unsafe {
            (0x6000000 as *mut u16)
                .write_volatile(color.convert_rgb::<EmuRsColorFormatRgb565>().raw());
        };
    }
}
