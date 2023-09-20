use core::fmt::Debug;
use core::mem::size_of;

use crate::driver::EmuRsDriver;
use alloc::vec::Vec;
use modular_bitfield::prelude::*;

use nalgebra::{ComplexField, DMatrix, Scalar};
use nalgebra::{Point2, Vector2};
use paste::paste;

pub const GNU_UNIFONT: EmuRsPsfFont = EmuRsPsfFont {
    data: include_bytes!("../../font/Unifont-APL8x16-15.1.01.psf"),
};

// Note that this is all based upon embedded-graphic's color implementation, but with more macro mess. Credit to them
// The macros are such a mess that when macros are expanded, this file is around 2000 lines with common rgb and bgr formats implemented!
// This isn't a problem unless you are compiling on a bad cpu

// TODO: Make some kind of solution for paletted displays
// TODO: Add RGBI type

/// Creates a new rgb color with its internal representation, and the width of the colors
#[macro_export]
macro_rules! rgb_color {
    ($internal_representation:ty, $r:expr, $g:expr, $b:expr) => {
        paste! {
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            #[repr(transparent)]
            pub struct [<EmuRsColorFormatRgb $r $g $b>] {
                pub data: $internal_representation,
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
                    // HACK: Find a better way to do this
                    let rgb888 = self.convert_rgb::<EmuRsColorFormatRgb888>();
                    let luma = luma(rgb888.red(), rgb888.blue(), rgb888.green());
                    return COLOR::new(luma);
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
                    debug_assert!($r + $g + $b <= size_of::<Self::InternalRepresentation>() * 8);

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
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
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
                    // HACK: Find a better way to do this
                    let rgb888 = self.convert_rgb::<EmuRsColorFormatRgb888>();
                    let luma = luma(rgb888.red(), rgb888.blue(), rgb888.green());
                    return COLOR::new(luma);
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
                    debug_assert!($b + $g + $r <= size_of::<Self::InternalRepresentation>() * 8);

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
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
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
                    let red = convert_channel(self.luma(), Self::MAX, COLOR::RMAX);
                    let green = convert_channel(self.luma(), Self::MAX, COLOR::GMAX);
                    let blue = convert_channel(self.luma(), Self::MAX, COLOR::BMAX);

                    return COLOR::new(red, green, blue);
                }

                fn convert_bgr<COLOR: EmuRsBgrColor>(&self) -> COLOR {
                    let blue = convert_channel(self.luma(), Self::MAX, COLOR::BMAX);
                    let green = convert_channel(self.luma(), Self::MAX, COLOR::GMAX);
                    let red = convert_channel(self.luma(), Self::MAX, COLOR::RMAX);

                    return COLOR::new(blue, green, red);
                }

                fn convert_grey<COLOR: EmuRsGreyColor>(&self) -> COLOR
                {
                    let luma = convert_channel(self.luma(), Self::MAX, COLOR::MAX);
                    return COLOR::new(luma);
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
                    debug_assert!($l <= size_of::<Self::InternalRepresentation>() * 8);

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
bgr_color!(u8, 1, 1, 1);

rgb_color!(u8, 2, 2, 2);
bgr_color!(u8, 2, 2, 2);

rgb_color!(u8, 3, 3, 2);
bgr_color!(u8, 3, 3, 2);

rgb_color!(u16, 3, 3, 3);
bgr_color!(u16, 3, 3, 3);

rgb_color!(u16, 4, 4, 4);
bgr_color!(u16, 4, 4, 4);

rgb_color!(u16, 5, 5, 5);
bgr_color!(u16, 5, 5, 5);

rgb_color!(u16, 5, 6, 5);
bgr_color!(u16, 5, 6, 5);

rgb_color!(u32, 6, 6, 6);
bgr_color!(u32, 6, 6, 6);

rgb_color!(u32, 7, 7, 7);
bgr_color!(u32, 7, 7, 7);

rgb_color!(u32, 8, 8, 8);
bgr_color!(u32, 8, 8, 8);

grey_color!(u8, 1);
grey_color!(u8, 2);
grey_color!(u8, 4);
grey_color!(u8, 8);

/// The backend for a color format
/// This allows color conversions and generic drawing functions
pub trait EmuRsColor: Clone + Copy + PartialEq {
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

pub trait EmuRsGreyColor: EmuRsColor {
    const MASK: usize;
    const MAX: usize;

    fn new(luma: u8) -> Self
    where
        Self: Sized;
    fn luma(&self) -> u8;
}

pub type EmuRsGenericColor = EmuRsColorFormatRgb888;

/// A video driver, with support for crude hardware acceleration that falls back to software methods
pub trait EmuRsVideoDriver: EmuRsDriver {
    /// Draw a single pixel
    fn draw_pixel(&mut self, color: EmuRsGenericColor, position: Point2<u16>);

    fn draw_texture(&mut self, texture: EmuRsTexture<EmuRsGenericColor>, position: Point2<u16>) {
        for x in 0..texture.data.ncols() {
            for y in 0..texture.data.nrows() {
                self.draw_pixel(
                    *texture.data.index((x, y)),
                    Point2::new(x as u16 + position.x, y as u16 + position.y),
                );
            }
        }
    }

    /// Draw a glyph on the screen
    fn draw_glyph(
        &mut self,
        _color: EmuRsGenericColor,
        _position: Point2<u16>,
        character: char,
        font: &dyn EmuRsFont,
    ) {
        let _font_data = font.get_char_glyph(character).unwrap();
        let _dimensions = font.get_dimensions();
    }

    /// Draw a line. This software implementation is rather slow at the moment
    fn draw_line(&mut self, color: EmuRsGenericColor, start: Point2<u16>, end: Point2<u16>) {
        let start_pos = Point2::new(start.x as isize, start.y as isize);
        let end_pos = Point2::new(end.x as isize, end.y as isize);

        if (end_pos.y - start_pos.y).abs() < (end_pos.x - start_pos.x).abs() {
            let mut plot_line_low =
                |start_pos: Point2<isize>, end_pos: Point2<isize>, color: EmuRsGenericColor| {
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
                        self.draw_pixel(color, Point2::new(x as u16, y as u16));
                        if d > 0 {
                            y = y + yi;
                            d = d + (2 * (dy - dx));
                        } else {
                            d = d + 2 * dy;
                        }
                    }
                };

            if start_pos.x > end_pos.x {
                plot_line_low(end_pos, start_pos, color);
            } else {
                plot_line_low(start_pos, end_pos, color);
            }
        } else {
            let mut plot_line_high =
                |start_pos: Point2<isize>, end_pos: Point2<isize>, color: EmuRsGenericColor| {
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
                        self.draw_pixel(color, Point2::new(x as u16, y as u16));
                        if d > 0 {
                            x = x + xi;
                            d = d + (2 * (dx - dy));
                        } else {
                            d = d + 2 * dx;
                        }
                    }
                };

            if start_pos.y > end_pos.y {
                plot_line_high(end_pos, start_pos, color);
            } else {
                plot_line_high(start_pos, end_pos, color);
            }
        }
    }

    /// Draw a polyline from a array of points
    fn draw_polyline(&mut self, color: EmuRsGenericColor, points: &[Point2<u16>], is_closed: bool) {
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

pub struct EmuRsTexture<COLOR: EmuRsColor + Scalar> {
    pub data: DMatrix<COLOR>,
}

impl<COLOR: EmuRsColor + Scalar> EmuRsTexture<COLOR> {
    pub fn new(data: DMatrix<COLOR>) -> Self {
        return Self { data };
    }

    pub fn convert_rgb<OTHER_COLOR: EmuRsRgbColor + Scalar>(&self) -> EmuRsTexture<OTHER_COLOR> {
        let data = self.data.map(|color| {
            return color.convert_rgb::<OTHER_COLOR>();
        });

        return EmuRsTexture { data };
    }

    pub fn convert_bgr<OTHER_COLOR: EmuRsBgrColor + Scalar>(&self) -> EmuRsTexture<OTHER_COLOR> {
        let data = self.data.map(|color| {
            return color.convert_bgr::<OTHER_COLOR>();
        });

        return EmuRsTexture { data };
    }

    pub fn convert_grey<OTHER_COLOR: EmuRsGreyColor + Scalar>(&self) -> EmuRsTexture<OTHER_COLOR> {
        let data = self.data.map(|color| {
            return color.convert_grey::<OTHER_COLOR>();
        });

        return EmuRsTexture { data };
    }
}

#[bitfield]
struct PsfFontFlags {
    pub is_512_character: B1,
    pub has_unicode_table: B1,
    pub modeseq: B1,
    extra: B5,
}

pub trait EmuRsFont {
    fn get_dimensions(&self) -> Vector2<u8>;
    fn get_char_glyph(&self, character: char) -> Option<&[u8]>;
    fn unicode_support(&self) -> bool;
}

pub struct EmuRsPsfFont<'owner> {
    pub data: &'owner [u8],
}

impl<'owner> EmuRsPsfFont<'owner> {
    fn version(&self) -> u8 {
        if u16::from_be_bytes(self.data[0..1].try_into().unwrap()) == 0x0436 {
            return 1;
        }

        if u32::from_be_bytes(self.data[0..3].try_into().unwrap()) == 0x864ab572 {
            return 2;
        }

        unreachable!();
    }
}

impl<'owner> EmuRsFont for EmuRsPsfFont<'owner> {
    fn get_dimensions(&self) -> Vector2<u8> {
        if self.version() == 1 {
            return Vector2::new(8, self.data[3]);
        }

        todo!()
    }

    /// To be honest this is what i think will work
    fn get_char_glyph(&self, character: char) -> Option<&[u8]> {
        let dim = self.get_dimensions();

        // FIXME: Test this extensively
        let glyph = character as usize + (dim.x as usize * dim.y as usize) + 4;

        return Some(&self.data[glyph..(glyph + (dim.x as usize * dim.y as usize))]);
    }

    fn unicode_support(&self) -> bool {
        todo!()
    }
}

/// Convert a color channel to some kind of other color channel
#[inline]
fn convert_channel(value: u8, from: usize, to: usize) -> u8 {
    if to == from {
        return value;
    }

    return (value as usize * (from / to)).min(to) as u8;
}

#[inline]
fn luma(r: u8, g: u8, b: u8) -> u8 {
    return (0.299 * (r as f32) + 0.587 * (g as f32) + 0.144 * (b as f32)) as u8;
}
