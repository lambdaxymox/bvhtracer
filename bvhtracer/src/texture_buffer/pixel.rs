use num_traits::{
    Bounded,
    NumCast,
    Num,
};

use std::ops;


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

    fn from_slice(slice: &[Self::Subpixel]) -> &Self;

    fn from_slice_mut(slice: &mut [Self::Subpixel]) -> &mut Self;

    fn to_rgb(&self) -> Rgb<Self::Subpixel>;

    fn to_rgba(&self) -> Rgba<Self::Subpixel>;

    fn map<Op>(&self, op: Op) -> Self where Op: FnMut(Self::Subpixel) -> Self::Subpixel;

    fn apply<Op>(&mut self, op: Op) where Op: FnMut(Self::Subpixel) -> Self::Subpixel;

    fn map_with_alpha<Op1, Op2>(&self, op1: Op1, op2: Op2) -> Self
    where
        Op1: FnMut(Self::Subpixel) -> Self::Subpixel,
        Op2: FnMut(Self::Subpixel) -> Self::Subpixel;

    fn apply_with_alpha<Op1, Op2>(&mut self, op1: Op1, op2: Op2)
    where
        Op1: FnMut(Self::Subpixel) -> Self::Subpixel,
        Op2: FnMut(Self::Subpixel) -> Self::Subpixel;

    fn invert(&self) -> Self;
    
    fn invert_mut(&mut self);

    fn blend(&self, other: &Self) -> Self;

    fn blend_mut(&mut self, other: &Self);

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

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Rgb<T> {
    data: [T; 3],
}

impl<T> Rgb<T>
where 
    T: Primitive,
{
    #[inline]
    pub fn new(r: T, g: T, b: T) -> Self {
        Self { 
            data: [r, g, b],
        }
    }

    #[inline]
    pub const fn r(&self) -> T {
        self.data[0]
    }

    #[inline]
    pub const fn g(&self) -> T {
        self.data[1]
    }

    #[inline]
    pub const fn b(&self) -> T {
        self.data[2]
    }
}

impl<T> From<[T; 3]> for Rgb<T>
where
    T: Primitive
{
    #[inline]
    fn from(data: [T; 3]) -> Self {
        Self { data, }
    }
}

impl<T> From<&[T; 3]> for Rgb<T>
where
    T: Primitive
{
    #[inline]
    fn from(data: &[T; 3]) -> Self {
        Self { data: *data, }
    }
}

impl<T> ops::Index<usize> for Rgb<T>
where
    T: Primitive
{
    type Output = T;

    #[inline]
    fn index(&self, _index: usize) -> &Self::Output {
        &self.data[_index]
    }
}

impl<T> ops::IndexMut<usize> for Rgb<T>
where
    T: Primitive
{
    #[inline]
    fn index_mut(&mut self, _index: usize) -> &mut Self::Output {
        &mut self.data[_index]
    }
}


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Rgba<T> {
    data: [T; 4],
}

impl<T> Rgba<T> 
where
    T: Primitive,
{
    #[inline]
    pub fn new(r: T, g: T, b: T, a: T) -> Self {
        Self { 
            data: [r, g, b, a],
        }
    }

    #[inline]
    pub const fn r(&self) -> T {
        self.data[0]
    }

    #[inline]
    pub const fn g(&self) -> T {
        self.data[1]
    }

    #[inline]
    pub const fn b(&self) -> T {
        self.data[2]
    }

    #[inline]
    pub const fn a(&self) -> T {
        self.data[3]
    }
}

impl<T> From<[T; 4]> for Rgba<T>
where
    T: Primitive
{
    #[inline]
    fn from(data: [T; 4]) -> Self {
        Self { data, }
    }
}

impl<T> From<&[T; 4]> for Rgba<T>
where
    T: Primitive
{
    #[inline]
    fn from(data: &[T; 4]) -> Self {
        Self { data: *data, }
    }
}

impl<T> ops::Index<usize> for Rgba<T>
where
    T: Primitive
{
    type Output = T;

    #[inline]
    fn index(&self, _index: usize) -> &Self::Output {
        &self.data[_index]
    }
}

impl<T> ops::IndexMut<usize> for Rgba<T>
where
    T: Primitive
{
    #[inline]
    fn index_mut(&mut self, _index: usize) -> &mut Self::Output {
        &mut self.data[_index]
    }
}


impl<T> Pixel for Rgb<T> 
where
    T: Primitive
{
    type Subpixel = T;

    const CHANNEL_COUNT: u8 = 3;
    const COLOR_MODEL: &'static str = "RGB";

    fn channels(&self) -> &[Self::Subpixel] {
        &self.data
    }

    fn channels_mut(&mut self) -> &mut [Self::Subpixel] {
        &mut self.data
    }

    fn from_slice(slice: &[Self::Subpixel]) -> &Self {
        assert_eq!(slice.len(), Self::CHANNEL_COUNT as usize);
        unsafe { 
            &*(slice.as_ptr() as *const Self)
        }
    }

    fn from_slice_mut(slice: &mut [Self::Subpixel]) -> &mut Self {
        assert_eq!(slice.len(), Self::CHANNEL_COUNT as usize);
        unsafe { 
            &mut *(slice.as_mut_ptr() as *mut Self)
        }
    }

    fn to_rgb(&self) -> Rgb<Self::Subpixel> {
        *self
    }

    fn to_rgba(&self) -> Rgba<Self::Subpixel> {
        Rgba::new(self.r(), self.g(), self.b(), <T as Primitive>::DEFAULT_MAX_VALUE)
    }

    fn map<Op>(&self, op: Op) -> Self where Op: FnMut(Self::Subpixel) -> Self::Subpixel {
        Self { data: self.data.map(op) }
    }

    fn apply<Op>(&mut self, op: Op) where Op: FnMut(Self::Subpixel) -> Self::Subpixel {
        self.data = self.data.map(op);
    }

    fn map_with_alpha<Op1, Op2>(&self, op1: Op1, _op2: Op2) -> Self
    where
        Op1: FnMut(Self::Subpixel) -> Self::Subpixel,
        Op2: FnMut(Self::Subpixel) -> Self::Subpixel
    {
        self.map(op1)
    }

    fn apply_with_alpha<Op1, Op2>(&mut self, op1: Op1, _op2: Op2)
    where
        Op1: FnMut(Self::Subpixel) -> Self::Subpixel,
        Op2: FnMut(Self::Subpixel) -> Self::Subpixel
    {
        self.apply(op1);
    }

    fn invert(&self) -> Self {
        let r = <T as Primitive>::DEFAULT_MAX_VALUE - self.r();
        let g = <T as Primitive>::DEFAULT_MAX_VALUE - self.g();
        let b = <T as Primitive>::DEFAULT_MAX_VALUE - self.b();

        Self::new(r, g, b)
    }
    
    fn invert_mut(&mut self) {
        self.apply(|chan| { <T as Primitive>::DEFAULT_MAX_VALUE - chan })
    }

    fn blend(&self, other: &Self) -> Self {
        unimplemented!()
    }

    fn blend_mut(&mut self, other: &Self) {
        unimplemented!()
    }
}

impl<T> Pixel for Rgba<T> 
where
    T: Primitive
{
    type Subpixel = T;

    const CHANNEL_COUNT: u8 = 4;
    const COLOR_MODEL: &'static str = "RGBA";

    fn channels(&self) -> &[Self::Subpixel] {
        &self.data
    }

    fn channels_mut(&mut self) -> &mut [Self::Subpixel] {
        &mut self.data
    }

    fn from_slice(slice: &[Self::Subpixel]) -> &Self {
        assert_eq!(slice.len(), Self::CHANNEL_COUNT as usize);
        unsafe { 
            &*(slice.as_ptr() as *const Self)
        }
    }

    fn from_slice_mut(slice: &mut [Self::Subpixel]) -> &mut Self {
        assert_eq!(slice.len(), Self::CHANNEL_COUNT as usize);
        unsafe { 
            &mut *(slice.as_mut_ptr() as *mut Self)
        }
    }

    fn to_rgb(&self) -> Rgb<Self::Subpixel> {
        Rgb::new(self.r(), self.g(), self.b())
    }

    fn to_rgba(&self) -> Rgba<Self::Subpixel> {
        *self
    }

    fn map<Op>(&self, op: Op) -> Self where Op: FnMut(Self::Subpixel) -> Self::Subpixel {
        Self { data: self.data.map(op) }
    }

    fn apply<Op>(&mut self, op: Op) where Op: FnMut(Self::Subpixel) -> Self::Subpixel {
        self.data = self.data.map(op);
    }

    fn map_with_alpha<Op1, Op2>(&self, mut op1: Op1, mut op2: Op2) -> Self
    where
        Op1: FnMut(Self::Subpixel) -> Self::Subpixel,
        Op2: FnMut(Self::Subpixel) -> Self::Subpixel
    {
        Rgba::from([op1(self.data[0]), op1(self.data[1]), op1(self.data[2]), op2(self.data[3])])
    }

    fn apply_with_alpha<Op1, Op2>(&mut self, mut op1: Op1, mut op2: Op2)
    where
        Op1: FnMut(Self::Subpixel) -> Self::Subpixel,
        Op2: FnMut(Self::Subpixel) -> Self::Subpixel
    {
        self.data[0] = op1(self.data[0]);
        self.data[1] = op1(self.data[1]);
        self.data[2] = op1(self.data[2]);
        self.data[3] = op2(self.data[3]);
    }

    fn invert(&self) -> Self {
        let r = <T as Primitive>::DEFAULT_MAX_VALUE - self.r();
        let g = <T as Primitive>::DEFAULT_MAX_VALUE - self.g();
        let b = <T as Primitive>::DEFAULT_MAX_VALUE - self.b();

        Self::new(r, g, b, self.a())
    }
    
    fn invert_mut(&mut self) {
        self.apply(|chan| { <T as Primitive>::DEFAULT_MAX_VALUE - chan })
    }

    fn blend(&self, other: &Self) -> Self {
        unimplemented!()
    }

    fn blend_mut(&mut self, other: &Self) {
        unimplemented!()
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Luma<T> {
    data: [T; 1],
}

impl<T> Luma<T> 
where
    T: Primitive,
{
    #[inline]
    pub fn new(y: T) -> Self {
        Self { 
            data: [y],
        }
    }

    #[inline]
    pub const fn y(&self) -> T {
        self.data[0]
    }
}

impl<T> From<[T; 1]> for Luma<T>
where
    T: Primitive
{
    #[inline]
    fn from(data: [T; 1]) -> Self {
        Self { data, }
    }
}

impl<T> From<&[T; 1]> for Luma<T>
where
    T: Primitive
{
    #[inline]
    fn from(data: &[T; 1]) -> Self {
        Self { data: *data, }
    }
}

impl<T> ops::Index<usize> for Luma<T>
where
    T: Primitive
{
    type Output = T;

    #[inline]
    fn index(&self, _index: usize) -> &Self::Output {
        &self.data[_index]
    }
}

impl<T> ops::IndexMut<usize> for Luma<T>
where
    T: Primitive
{
    #[inline]
    fn index_mut(&mut self, _index: usize) -> &mut Self::Output {
        &mut self.data[_index]
    }
}

impl<T> Pixel for Luma<T> 
where
    T: Primitive
{
    type Subpixel = T;

    const CHANNEL_COUNT: u8 = 1;
    const COLOR_MODEL: &'static str = "Y";

    fn channels(&self) -> &[Self::Subpixel] {
        &self.data
    }

    fn channels_mut(&mut self) -> &mut [Self::Subpixel] {
        &mut self.data
    }

    fn from_slice(slice: &[Self::Subpixel]) -> &Self {
        assert_eq!(slice.len(), Self::CHANNEL_COUNT as usize);
        unsafe { 
            &*(slice.as_ptr() as *const Self)
        }
    }

    fn from_slice_mut(slice: &mut [Self::Subpixel]) -> &mut Self {
        assert_eq!(slice.len(), Self::CHANNEL_COUNT as usize);
        unsafe { 
            &mut *(slice.as_mut_ptr() as *mut Self)
        }
    }

    fn to_rgb(&self) -> Rgb<Self::Subpixel> {
        Rgb::from([self.y(), self.y(), self.y()])
    }

    fn to_rgba(&self) -> Rgba<Self::Subpixel> {
        Rgba::new(self.y(), self.y(), self.y(), <T as Primitive>::DEFAULT_MAX_VALUE)
    }

    fn map<Op>(&self, op: Op) -> Self where Op: FnMut(Self::Subpixel) -> Self::Subpixel {
        Self { data: self.data.map(op) }
    }

    fn apply<Op>(&mut self, op: Op) where Op: FnMut(Self::Subpixel) -> Self::Subpixel {
        self.data = self.data.map(op);
    }

    fn map_with_alpha<Op1, Op2>(&self, op1: Op1, _op2: Op2) -> Self
    where
        Op1: FnMut(Self::Subpixel) -> Self::Subpixel,
        Op2: FnMut(Self::Subpixel) -> Self::Subpixel
    {
        self.map(op1)
    }

    fn apply_with_alpha<Op1, Op2>(&mut self, op1: Op1, _op2: Op2)
    where
        Op1: FnMut(Self::Subpixel) -> Self::Subpixel,
        Op2: FnMut(Self::Subpixel) -> Self::Subpixel
    {
        self.apply(op1);
    }

    fn invert(&self) -> Self {
        let y = <T as Primitive>::DEFAULT_MAX_VALUE - self.y();

        Self::new(y)
    }
    
    fn invert_mut(&mut self) {
        self.apply(|chan| { <T as Primitive>::DEFAULT_MAX_VALUE - chan })
    }

    fn blend(&self, other: &Self) -> Self {
        unimplemented!()
    }

    fn blend_mut(&mut self, other: &Self) {
        unimplemented!()
    }
}