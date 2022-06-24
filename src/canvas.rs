use num_traits::{
    Bounded,
    NumCast,
    Num,
};

use std::ops;


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Rgb<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> Rgb<T>
where 
    T: Primitive,
{
    #[inline]
    pub fn new(r: T, g: T, b: T) -> Self {
        Self { 
            r, b, g,
        }
    }

    #[inline]
    pub const fn zero() -> Self {
        Self { 
            r: <T as Primitive>::DEFAULT_MIN_VALUE, 
            g: <T as Primitive>::DEFAULT_MIN_VALUE, 
            b: <T as Primitive>::DEFAULT_MIN_VALUE, 
        }
    }
}


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Rgba<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T> Rgba<T> 
where
    T: Primitive,
{
    #[inline]
    pub fn new(r: T, g: T, b: T, a: T) -> Self {
        Self { 
            r, b, g, a,
        }
    }

    #[inline]
    pub const fn zero() -> Self {
        Self { 
            r: <T as Primitive>::DEFAULT_MIN_VALUE, 
            g: <T as Primitive>::DEFAULT_MIN_VALUE, 
            b: <T as Primitive>::DEFAULT_MIN_VALUE,
            a: <T as Primitive>::DEFAULT_MAX_VALUE,
        }
    }
}

/// The bottom-level property of a pixel--namely, the type of each pixel channel.
pub trait Primitive: Copy + NumCast + Num + PartialOrd<Self> + Clone + Bounded {
    const DEFAULT_MAX_VALUE: Self;
    const DEFAULT_MIN_VALUE: Self;
}

impl Primitive for usize {
    const DEFAULT_MAX_VALUE: Self = usize::MAX;
    const DEFAULT_MIN_VALUE: Self = usize::MIN;
}

impl Primitive for u8 {
    const DEFAULT_MAX_VALUE: Self = u8::MAX;
    const DEFAULT_MIN_VALUE: Self = u8::MIN;
}

impl Primitive for u16 {
    const DEFAULT_MAX_VALUE: Self = u16::MAX;
    const DEFAULT_MIN_VALUE: Self = u16::MIN;
}

impl Primitive for u32 {
    const DEFAULT_MAX_VALUE: Self = u32::MAX;
    const DEFAULT_MIN_VALUE: Self = u32::MIN;
}

impl Primitive for u64 {
    const DEFAULT_MAX_VALUE: Self = u64::MAX;
    const DEFAULT_MIN_VALUE: Self = u64::MIN;
}

impl Primitive for isize {
    const DEFAULT_MAX_VALUE: Self = isize::MAX;
    const DEFAULT_MIN_VALUE: Self = isize::MIN;
}

impl Primitive for i8 {
    const DEFAULT_MAX_VALUE: Self = i8::MAX;
    const DEFAULT_MIN_VALUE: Self = i8::MIN;
}

impl Primitive for i16 {
    const DEFAULT_MAX_VALUE: Self = i16::MAX;
    const DEFAULT_MIN_VALUE: Self = i16::MIN;
}

impl Primitive for i32 {
    const DEFAULT_MAX_VALUE: Self = i32::MAX;
    const DEFAULT_MIN_VALUE: Self = i32::MIN;
}

impl Primitive for i64 {
    const DEFAULT_MAX_VALUE: Self = i64::MAX;
    const DEFAULT_MIN_VALUE: Self = i64::MIN;
}

impl Primitive for f32 {
    const DEFAULT_MAX_VALUE: Self = 1_f32;
    const DEFAULT_MIN_VALUE: Self = 0_f32;
}

impl Primitive for f64 {
    const DEFAULT_MAX_VALUE: Self = 1_f64;
    const DEFAULT_MIN_VALUE: Self = 0_f64;
}

pub trait Pixel: Copy + Clone {
    type Subpixel: Primitive;

    const CHANNEL_COUNT: u8;
    const COLOR_MODEL: &'static str;

    fn channels(&self) -> &[Self::Subpixel];

    fn channels_mut(&mut self) -> &mut [Self::Subpixel];

    fn to_rgb(&self) -> Rgb<Self::Subpixel>;

    fn to_rgba(&self) -> Rgba<Self::Subpixel>;

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
    pub data: Vec<Rgba<u8>>,
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

    pub fn as_ptr(&self) -> *const Rgba<u8> {
        self.data.as_ptr()
    }
}

impl ops::Index<usize> for Canvas {
    type Output = [Rgba<u8>];

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

