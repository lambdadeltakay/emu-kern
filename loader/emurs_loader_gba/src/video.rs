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
    pub struct DisplayControl(u16);
    pub background_mode, set_background_mode: 2, 0;
    pub cgb_mode_select, set_cgb_mode_select: 3;
    pub display_frame_select, set_display_frame_select: 4;
    pub hblank_interval_free, set_hblank_interval_free: 5;
    pub object_character_vram_wrapping, set_object_character_vram_wrapping: 6;
    pub forced_blank, set_forced_blank: 7;
    pub display_background_0, set_display_background_0: 8;
    pub display_background_1, set_display_background_1: 9;
    pub display_background_2, set_display_background_2: 10;
    pub display_background_3, set_display_background_3: 11;
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
        unsafe { DISPCNT.as_mut().unwrap().set_forced_blank(false) };
        unsafe { DISPCNT.as_mut().unwrap().set_display_background_2(true) };
    }
}

impl EmuRsVideoDriver for GbaVideo {
    fn draw_pixel(&mut self, color: impl EmuRsColor, position: Point2<usize>) {
        debug_assert!(position.x <= 240);
        debug_assert!(position.y <= 160);

        unsafe {
            (0x6000000 as *mut u16)
                .add(position.x + position.y * 240)
                .write_volatile(color.convert_rgb::<EmuRsColorFormatRgb565>().raw());
        };
    }
}
