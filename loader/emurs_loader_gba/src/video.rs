use alloc::rc::Rc;
use alloc::string::String;
use core::mem::size_of;
use core::ptr::write_volatile;
use emurs_kernel::device::EmuRsDevice;
use emurs_kernel::driver::*;
use emurs_kernel::prelude::lock_api::{Mutex, RawMutex};
use emurs_kernel::prelude::spin::mutex::spin;
use emurs_kernel::prelude::tinyvec::{array_vec, ArrayVec};
use emurs_kernel::video::{EmuRsColor, EmuRsColorFormatRgb565, EmuRsRgbColor};
use emurs_kernel::video::{EmuRsColorFormatBgr565, EmuRsGenericColor};
use emurs_kernel::EmuRsContext;
use emurs_kernel::{
    prelude::nalgebra::{Point2, Vector2},
    video::EmuRsVideoDriver,
};
use modular_bitfield::prelude::*;

#[bitfield]
#[repr(transparent)]
pub struct DisplayControl {
    pub background_mode: B3,
    pub cgb_mode_select: B1,
    pub display_frame_select: B1,
    pub hblank_interval_free: B1,
    pub object_character_vram_wrapping: B1,
    pub forced_blank: B1,
    pub display_background_0: B1,
    pub display_background_1: B1,
    pub display_background_2: B1,
    pub display_background_3: B1,
    pub display_background_obj: B1,
    pub window_display_flag_1: B1,
    pub windows_display_flag_2: B1,
    pub obj_window_display_flag: B1,
}

const DISPCNT: *mut DisplayControl = 0x4000000 as *mut DisplayControl;

#[derive(Default)]
pub struct GbaVideo;

impl EmuRsDriver for GbaVideo {
    fn name(&self) -> &str {
        return "Game Boy Advance Video Driver";
    }
    fn get_claimed(&mut self) -> EmuRsDevice {
        todo!()
    }

    fn init(&mut self, context: Rc<EmuRsContext>) {
        debug_assert_eq!(size_of::<DisplayControl>(), size_of::<u16>());

        let dispcnt = DisplayControl::new()
            .with_background_mode(3)
            .with_forced_blank(0)
            .with_display_background_2(1);
        unsafe { DISPCNT.write_volatile(dispcnt) };
    }

    fn get_preference(&mut self) -> EmuRsDriverPreference {
        return EmuRsDriverPreference::Preferred;
    }
}

impl EmuRsVideoDriver for GbaVideo {
    fn draw_pixel(&mut self, color: EmuRsGenericColor, position: Point2<u16>) {
        unsafe {
            let pixel_location = (0x6000000 as *mut EmuRsColorFormatBgr565)
                .add((position.x + position.y * 240).into());

            if pixel_location <= 0x6017fff as *mut EmuRsColorFormatBgr565 {
                pixel_location.write_volatile(color.convert_bgr());
            }
        };
    }
}
