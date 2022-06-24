use num_traits::{
    Bounded,
    NumCast,
    Num,
};

use std::ops;


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    #[inline]
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { 
            r, b, g,
        }
    }

    #[inline]
    pub const fn zero() -> Self {
        Self { r: 0, g: 0, b: 0, }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    #[inline]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { 
            r, b, g, a,
        }
    }

    #[inline]
    pub const fn zero() -> Self {
        Self { r: 0, g: 0, b: 0, a: 1 }
    }
}

/// The bottom-level property of a pixel--namely, the type of each pixel channel.
pub trait Primitive: Copy + NumCast + Num + PartialOrd<Self> + Clone + Bounded {
    const DEFAULT_MAX_VALUE: Self;
    const DEFAULT_MIN_VALUE: Self;
}

pub trait Pixel: Copy + Clone {
    type Subpixel: Primitive;

    const CHANNEL_COUNT: u8;
    const COLOR_MODEL: &'static str;

    fn channels(&self) -> &[Self::Subpixel];

    fn channels_mut(&mut self) -> &mut [Self::Subpixel];

    /*
    fn to_rgb(&self) -> Rgb<Self::Subpixel>;

    fn to_rgba(&self) -> Rgba<Self::Subpixel>;
    */

    fn map<Op>(&self, op: Op) -> Self where Op: FnMut(Self::Subpixel) -> Self::Subpixel;

    fn apply<Op>(&mut self, op: Op) where Op: FnMut(Self::Subpixel) -> Self::Subpixel;

    fn map_with_alpha<Op1, Op2>(&self, op1: Op1, op2: Op2) -> Self
    where
        Op1: FnMut(Self::Subpixel) -> Self::Subpixel,
        Op2: FnMut(Self::Subpixel) -> Self::Subpixel;

    fn apply_with_alpha<Op1, Op2>(&self, op1: Op1, op2: Op2) -> Self
    where
        Op1: FnMut(Self::Subpixel) -> Self::Subpixel,
        Op2: FnMut(Self::Subpixel) -> Self::Subpixel;

    fn invert(&mut self);

    fn blend(&mut self, other: &Self);

    fn map_without_alpha<Op>(&self, op: Op) -> Self 
    where 
        Op: FnMut(Self::Subpixel) -> Self::Subpixel 
    {
        let mut cloned = *self;
        cloned.apply_with_alpha(op, |alpha| alpha);
        
        cloned
    }

    fn apply_without_alpha<Op>(&mut self, op: Op)
    where
        Op: FnMut(Self::Subpixel) -> Self::Subpixel
    {
        self.apply_with_alpha(op, |alpha| alpha);
    }
}
/*
impl Pixel for Rgba {
    const CHANNEL_COUNT: u8 = 4;
    const COLOR_MODEL: &'static str = "RGBA";
}
*/

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Rgba>,
}

impl Canvas 
where
{
    pub fn new(width: usize, height: usize) -> Self {
        Self { 
            width, 
            height, 
            data: vec![Rgba::zero(); width * height],
        }
    }

    pub fn clear(&mut self) {
        let zero = Rgba::zero();
        for pixel in self.data.as_mut_slice() {
            *pixel = zero;
        }
    }

    pub fn as_ptr(&self) -> *const Rgba {
        self.data.as_ptr()
    }
}

impl ops::Index<usize> for Canvas {
    type Output = [Rgba];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        let row_start = index * self.width;
        let row_end = row_start + self.width;

        &self.data[row_start..row_end]
    }
}

impl ops::IndexMut<usize> for Canvas {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let row_start = index * self.width;
        let row_end = row_start + self.width;

        &mut self.data[row_start..row_end]
    }
}

