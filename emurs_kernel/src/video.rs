use core::mem::size_of;

use crate::driver::EmuRsDriver;
use nalgebra::{ComplexField, SVector};
use nalgebra::{Point2, Vector2};
use paste::paste;
use tinyvec::{ArrayVec, TinyVec};

// Note that this is all based upon embedded-graphic's color implementation, but with more macro mess. Credit to them
// The macros are such a mess that when macros are expanded, this file is around 3000 lines with common rgb and bgr formats implemented!
// This isn't a problem unless you are compiling on a bad cpu

// TODO: Make some kind of solution for paletted displays

/// Creates a new rgb color with its internal representation, and the width of the colors
#[macro_export]
macro_rules! rgb_color {
    ($internal_representation:ty, $r:expr, $g:expr, $b:expr) => {
        paste! {
            #[derive(Clone, Copy, Debug, Default)]
            #[repr(transparent)]
            pub struct [<EmuRsColorFormatRgb $r $g $b>] {
                pub data: <Self as EmuRsColor>::InternalRepresentation,
            }

            impl EmuRsColor for [<EmuRsColorFormatRgb $r $g $b>] {
                type InternalRepresentation = $internal_representation;

                fn raw(&self) -> Self::InternalRepresentation {
                    return self.data;
                }

                fn convert_rgb<COLOR: EmuRsRgbColor>(&self) -> COLOR {
                    let red = convert_channel(self.red(), Self::RMAX, COLOR::RMAX);
                    let green = convert_channel(self.green(), Self::GMAX, COLOR::GMAX);
                    let blue = convert_channel(self.blue(), Self::BMAX, COLOR::BMAX);

                    return COLOR::new(red, green, blue);
                }

                fn convert_bgr<COLOR: EmuRsBgrColor>(&self) -> COLOR {
                    let blue = convert_channel(self.blue(), Self::BMAX, COLOR::BMAX);
                    let green = convert_channel(self.green(), Self::GMAX, COLOR::GMAX);
                    let red = convert_channel(self.red(), Self::RMAX, COLOR::RMAX);

                    return COLOR::new(blue, green, red);
                }
                
                fn convert_grey<COLOR: EmuRsGreyColor>(&self) -> COLOR
                {
                    todo!()
                }

            }

            impl EmuRsRgbColor for [<EmuRsColorFormatRgb $r $g $b>] {
                const RMAX: usize = 2_usize.pow($r);
                const GMAX: usize = 2_usize.pow($g);
                const BMAX: usize = 2_usize.pow($b);

                const RMASK: usize = (Self::RMAX - 1) << ($g + $b);
                const GMASK: usize = (Self::GMAX - 1) << $b;
                const BMASK: usize = Self::BMAX - 1;

                fn new(red: u8, green: u8, blue: u8) -> Self
                where
                    Self: Sized,
                {
                    debug_assert!(red as usize <= Self::RMAX);
                    debug_assert!(green as usize <= Self::GMAX);
                    debug_assert!(blue as usize <= Self::BMAX);
                    debug_assert!($r + $g + $b <= size_of::<Self::InternalRepresentation>());

                    return Self {
                        data: (red as Self::InternalRepresentation) << ($g + $b)
                            | (green as Self::InternalRepresentation) << $b
                            | (blue as Self::InternalRepresentation),
                    };
                }

                fn red(&self) -> u8 {
                    return ((self.data as usize & Self::RMASK) >> ($g + $b)) as u8;
                }

                fn green(&self) -> u8 {
                    return ((self.data as usize & Self::GMASK) >> $b) as u8;
                }

                fn blue(&self) -> u8 {
                    return (self.data as usize & Self::BMASK) as u8;
                }
            }
        }
    };
}

/// Creates a new bgr color with its internal representation, and the width of the colors
#[macro_export]
macro_rules! bgr_color {
    ($internal_representation:ty, $b:expr, $g:expr, $r:expr) => {
        paste! {
            #[derive(Clone, Copy, Debug, Default)]
            #[repr(transparent)]
            pub struct [<EmuRsColorFormatBgr $b $g $r>] {
                pub data: <Self as EmuRsColor>::InternalRepresentation,
            }

            impl EmuRsColor for [<EmuRsColorFormatBgr $b $g $r>] {
                type InternalRepresentation = $internal_representation;

                fn raw(&self) -> Self::InternalRepresentation {
                    return self.data;
                }

                fn convert_rgb<COLOR: EmuRsRgbColor>(&self) -> COLOR {
                    let red = convert_channel(self.red(), Self::RMAX, COLOR::RMAX);
                    let green = convert_channel(self.green(), Self::GMAX, COLOR::GMAX);
                    let blue = convert_channel(self.blue(), Self::BMAX, COLOR::BMAX);

                    return COLOR::new(red, green, blue);
                }

                fn convert_bgr<COLOR: EmuRsBgrColor>(&self) -> COLOR {
                    let blue = convert_channel(self.blue(), Self::BMAX, COLOR::BMAX);
                    let green = convert_channel(self.green(), Self::GMAX, COLOR::GMAX);
                    let red = convert_channel(self.red(), Self::RMAX, COLOR::RMAX);

                    return COLOR::new(blue, green, red);
                }
                
                fn convert_grey<COLOR: EmuRsGreyColor>(&self) -> COLOR
                {
                    todo!()
                }
            }

            impl EmuRsBgrColor for [<EmuRsColorFormatBgr $b $g $r>] {
                const BMAX: usize = 2_usize.pow($b);
                const GMAX: usize = 2_usize.pow($g);
                const RMAX: usize = 2_usize.pow($r);

                const BMASK: usize = (Self::BMAX - 1) << ($g + $r);
                const GMASK: usize = (Self::GMAX - 1) << $r;
                const RMASK: usize = Self::RMAX - 1;

                fn new(blue: u8, green: u8, red: u8) -> Self
                where
                    Self: Sized,
                {
                    debug_assert!(blue as usize <= Self::BMAX);
                    debug_assert!(green as usize <= Self::GMAX);
                    debug_assert!(red as usize <= Self::RMAX);
                    debug_assert!($b + $g + $r <= size_of::<Self::InternalRepresentation>());

                    return Self {
                        data: (blue as Self::InternalRepresentation) << ($g + $r)
                            | (green as Self::InternalRepresentation) << $r
                            | (red as Self::InternalRepresentation),
                    };
                }

                fn blue(&self) -> u8 {
                    return ((self.data as usize & Self::BMASK) >> ($g + $r)) as u8;

                }

                fn green(&self) -> u8 {
                    return ((self.data as usize & Self::GMASK) >> $r) as u8;

                }

                fn red(&self) -> u8 {
                    return (self.data as usize & Self::RMASK) as u8;
                }
            }
        }
    };
}

#[macro_export]
macro_rules! grey_color {
    ($internal_representation:ty, $l:expr) => {
        paste! {
            #[derive(Clone, Copy, Debug, Default)]
            #[repr(transparent)]
            pub struct [<EmuRsColorFormatGrey $l>] {
                pub data: <Self as EmuRsColor>::InternalRepresentation,
            }

            impl EmuRsColor for [<EmuRsColorFormatGrey $l>]{
                type InternalRepresentation = $internal_representation;

                fn raw(&self) -> Self::InternalRepresentation {
                    return self.data;
                }

                fn convert_rgb<COLOR: EmuRsRgbColor>(&self) -> COLOR {
                    todo!()
                }

                fn convert_bgr<COLOR: EmuRsBgrColor>(&self) -> COLOR {
                    todo!()
                }
                
                fn convert_grey<COLOR: EmuRsGreyColor>(&self) -> COLOR
                {
                    todo!()
                }
            }

            impl EmuRsGreyColor for [<EmuRsColorFormatGrey $l>] {
                const MAX: usize = 2_usize.pow($l);
                const MASK: usize = (Self::MAX - 1);

                fn new(luma: u8) -> Self
                where
                    Self: Sized,
                {
                    debug_assert!(luma as usize <= Self::MAX);
                    debug_assert!($l <= size_of::<Self::InternalRepresentation>());

                    return Self {
                        data: $l as Self::InternalRepresentation
                    };
                }

                fn luma(&self) -> u8 {
                    return self.data as u8
                }
            }
        }
    };
}

rgb_color!(u8, 1, 1, 1);
rgb_color!(u8, 2, 2, 2);
rgb_color!(u8, 3, 3, 2);
rgb_color!(u16, 3, 3, 3);
rgb_color!(u16, 4, 4, 4);
rgb_color!(u16, 5, 5, 5);
rgb_color!(u16, 5, 6, 5);
rgb_color!(u32, 6, 6, 6);
rgb_color!(u32, 7, 7, 7);
rgb_color!(u32, 8, 8, 8);

bgr_color!(u8, 1, 1, 1);
bgr_color!(u8, 2, 2, 2);
bgr_color!(u8, 3, 3, 2);
bgr_color!(u16, 3, 3, 3);
bgr_color!(u16, 4, 4, 4);
bgr_color!(u16, 5, 5, 5);
bgr_color!(u16, 5, 6, 5);
bgr_color!(u32, 6, 6, 6);
bgr_color!(u32, 7, 7, 7);
bgr_color!(u32, 8, 8, 8);

grey_color!(u8, 1);
grey_color!(u8, 2);
grey_color!(u8, 4);
grey_color!(u8, 8);

/// The backend for a color format
/// This allows color conversions and generic drawing functions
pub trait EmuRsColor: Clone + Copy {
    type InternalRepresentation;

    fn raw(&self) -> Self::InternalRepresentation;
    fn convert_rgb<COLOR: EmuRsRgbColor>(&self) -> COLOR;
    fn convert_bgr<COLOR: EmuRsBgrColor>(&self) -> COLOR;
    fn convert_grey<COLOR: EmuRsGreyColor>(&self) -> COLOR;
}

/// The backend implementation for a RGB based color
pub trait EmuRsRgbColor: EmuRsColor {
    const RMAX: usize;
    const GMAX: usize;
    const BMAX: usize;
    const RMASK: usize;
    const GMASK: usize;
    const BMASK: usize;

    fn new(red: u8, green: u8, blue: u8) -> Self
    where
        Self: Sized;
    fn red(&self) -> u8;
    fn green(&self) -> u8;
    fn blue(&self) -> u8;
}

/// The backend implementation for a BGR based color
pub trait EmuRsBgrColor: EmuRsColor {
    const BMAX: usize;
    const GMAX: usize;
    const RMAX: usize;
    const BMASK: usize;
    const GMASK: usize;
    const RMASK: usize;

    fn new(blue: u8, green: u8, red: u8) -> Self
    where
        Self: Sized;
    fn blue(&self) -> u8;
    fn green(&self) -> u8;
    fn red(&self) -> u8;
}

pub trait EmuRsGreyColor: EmuRsColor
{
    const MASK: usize;
    const MAX: usize;
    
    fn new(luma: u8) -> Self
    where
        Self: Sized;
    fn luma(&self) -> u8;
}

/// A video driver, with support for crude hardware acceleration that falls back to software methods
pub trait EmuRsVideoDriver: EmuRsDriver {
    /// Draw a single pixel. The only method that does not have a software implementation
    fn draw_pixel(&mut self, color: impl EmuRsColor, position: Point2<usize>);

    /// Draw a line. This software implementation is rather slow at the moment
    fn draw_line(&mut self, color: impl EmuRsColor, start: Point2<usize>, end: Point2<usize>) {
        fn plot_line_low(
            context: &mut (impl EmuRsVideoDriver + ?Sized),
            start_pos: Point2<isize>,
            end_pos: Point2<isize>,
            color: impl EmuRsColor,
        ) {
            let dx = end_pos.x - start_pos.x;
            let mut dy = end_pos.y - start_pos.y;
            let mut yi = 1;
            if dy < 0 {
                yi = -1;
                dy = -dy;
            }
            let mut d = (2 * dy) - dx;
            let mut y = start_pos.y;
            for x in start_pos.x..=end_pos.x {
                context.draw_pixel(color, Point2::new(x as usize, y as usize));
                if d > 0 {
                    y = y + yi;
                    d = d + (2 * (dy - dx));
                } else {
                    d = d + 2 * dy;
                }
            }
        }

        fn plot_line_high(
            context: &mut (impl EmuRsVideoDriver + ?Sized),
            start_pos: Point2<isize>,
            end_pos: Point2<isize>,
            color: impl EmuRsColor,
        ) {
            let mut dx = end_pos.x - start_pos.x;
            let dy = end_pos.y - start_pos.y;
            let mut xi = 1;
            if dx < 0 {
                xi = -1;
                dx = -dx;
            }
            let mut d = (2 * dx) - dy;
            let mut x = start_pos.x;
            for y in start_pos.y..=end_pos.y {
                context.draw_pixel(color, Point2::new(x as usize, y as usize));
                if d > 0 {
                    x = x + xi;
                    d = d + (2 * (dx - dy));
                } else {
                    d = d + 2 * dx;
                }
            }
        }

        let start_pos = Point2::new(start.x as isize, start.y as isize);
        let end_pos = Point2::new(end.x as isize, end.y as isize);

        if (end_pos.y - start_pos.y).abs() < (end_pos.x - start_pos.x).abs() {
            if start_pos.x > end_pos.x {
                plot_line_low(self, end_pos, start_pos, color);
            } else {
                plot_line_low(self, start_pos, end_pos, color);
            }
        } else {
            if start_pos.y > end_pos.y {
                plot_line_high(self, end_pos, start_pos, color);
            } else {
                plot_line_high(self, start_pos, end_pos, color);
            }
        }
    }

    /// Draw a polyline from a fixed array
    ///
    /// To use this easily just pass in a fixed array to [SVector]
    ///
    fn draw_polyline(
        &mut self,
        points: &[Point2<usize>],
        color: impl EmuRsColor,
        is_closed: bool,
    ) {
        // Handle easily optimizable functions
        match points.len() {
            0 => {
                return;
            }
            1 => {
                self.draw_pixel(color, points[0]);
                return;
            }
            2 => {
                self.draw_line(color, points[0], points[1]);
                return;
            }
            _ => (),
        }

        let mut last_point = None;
        for point in points.iter() {
            if last_point.is_some() {
                self.draw_line(color, last_point.unwrap(), *point);
            }

            last_point = Some(*point);
        }
        if is_closed {
            let first_point = points.iter().nth(0).unwrap();
            self.draw_line(color, *first_point, last_point.unwrap());
        }
    }
}

/// Convert a color channel to some kind of other color channel
/// FIXME: The math here needs to be made without floats
/// FIXME: The math here does not work at all.
#[inline]
fn convert_channel(value: u8, from: usize, to: usize) -> u8 {
    if to == from {
        return value;
    }

    return (value as f32).scale(to as f32 / from as f32).round() as u8;
}

fn luma(r: u8, g: u8, b: u8) -> u8
{
    return (0.299 * (r as f32) + 0.587 * (g as f32) + 0.144 * (b as f32)) as u8;
}

/// A dummy driver, for devices that have no displays
pub struct EmuRsDummyVideoDriver;

impl EmuRsDriver for EmuRsDummyVideoDriver {
    fn name(&self) -> &str {
        return "Dummy Video Driver";
    }

    fn get_claimed(&self) -> crate::device::EmuRsDevice {
        todo!()
    }

    fn setup_hardware(&self) {}
}

impl EmuRsVideoDriver for EmuRsDummyVideoDriver {
    fn draw_pixel(&mut self, color: impl EmuRsColor, position: Point2<usize>) {}
}
