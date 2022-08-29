use crate::texture_buffer::pixel::*;

use num_traits::{
    Zero,
};
use std::ops;
use std::marker::{
    PhantomData,
};
use std::slice::{
    ChunksExact,
    ChunksExactMut,
};


pub struct Pixels<'a, P: Pixel + 'a>
where
    P::Subpixel: 'a,
{
    chunks: ChunksExact<'a, P::Subpixel>,
}

impl<'a, P: Pixel + 'a> Iterator for Pixels<'a, P> {
    type Item = &'a P;

    #[inline(always)]
    fn next(&mut self) -> Option<&'a P> {
        self.chunks.next().map(|v| <P as Pixel>::from_slice(v))
    }
}

impl<'a, P: Pixel + 'a> Clone for Pixels<'a, P> {
    fn clone(&self) -> Self {
        Pixels { 
            chunks: self.chunks.clone(), 
        }
    }
}

pub struct PixelsMut<'a, P: Pixel + 'a>
where
    P::Subpixel: 'a,
{
    chunks: ChunksExactMut<'a, P::Subpixel>,
}

impl<'a, P: Pixel + 'a> Iterator for PixelsMut<'a, P>
where
    P::Subpixel: 'a,
{
    type Item = &'a mut P;

    #[inline(always)]
    fn next(&mut self) -> Option<&'a mut P> {
        self.chunks.next().map(|v| <P as Pixel>::from_slice_mut(v))
    }
}

pub struct EnumeratePixels<'a, P: Pixel + 'a>
where
    <P as Pixel>::Subpixel: 'a,
{
    pixels: Pixels<'a, P>,
    x: usize,
    y: usize,
    width: usize,
}

impl<'a, P: Pixel + 'a> Iterator for EnumeratePixels<'a, P>
where
    <P as Pixel>::Subpixel: 'a,
{
    type Item = (usize, usize, &'a P);
    
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.width {
            self.x = 0;
            self.y += 1;
        }
        let (x, y) = (self.x, self.y);
        self.x += 1;
        self.pixels.next().map(|pixel| (x, y, pixel))
    }
}

pub struct EnumeratePixelsMut<'a, P: Pixel + 'a>
where
    <P as Pixel>::Subpixel: 'a,
{
    pixels: PixelsMut<'a, P>,
    x: usize,
    y: usize,
    width: usize,
}

impl<'a, P: Pixel + 'a> Iterator for EnumeratePixelsMut<'a, P>
where
    <P as Pixel>::Subpixel: 'a,
{
    type Item = (usize, usize, &'a mut P);
    
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.width {
            self.x = 0;
            self.y += 1;
        }
        let (x, y) = (self.x, self.y);
        self.x += 1;
        self.pixels.next().map(|pixel| (x, y, pixel))
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct TextureBuffer2D<P, Storage> {
    width: usize,
    height: usize,
    data: Storage,
    _marker: PhantomData<P>,
}

impl<P, Storage> TextureBuffer2D<P, Storage>
where
    P: Pixel,
    Storage: ops::Deref<Target = [P::Subpixel]>,
{
    pub fn from_raw(width: usize, height: usize, buffer: Storage) -> Option<TextureBuffer2D<P, Storage>> {
        if Self::image_fits_inside_storage(width, height, buffer.len()) {
            Some(TextureBuffer2D {
                width,
                height,
                data: buffer,
                _marker: PhantomData,
            })
        } else {
            None
        }
    }

    pub fn from_vec(
        width: usize, 
        height: usize, 
        buffer: Vec<P::Subpixel>
    ) -> Option<TextureBuffer2D<P, Vec<P::Subpixel>>> 
    {
        TextureBuffer2D::from_raw(width, height, buffer)
    }

    pub fn pixels(&self) -> Pixels<P> {
        let len = Self::image_buffer_len(self.width, self.height).unwrap();
        let subpixels = &self.data[..len];

        Pixels {
            chunks: subpixels.chunks_exact(<P as Pixel>::CHANNEL_COUNT as usize),
        }
    }

    pub fn enumerate_pixels(&self) -> EnumeratePixels<P> {
        EnumeratePixels {
            pixels: self.pixels(),
            x: 0,
            y: 0,
            width: self.width,
        }
    }

    #[inline]
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn width_subpixels(&self) -> usize {
        self.width * (P::CHANNEL_COUNT as usize)
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    fn image_fits_inside_storage(width: usize, height: usize, length: usize) -> bool {
        let checked_length = Self::image_buffer_len(width, height);
        checked_length.map(|min_len| min_len <= length).unwrap_or(false)
    }

    fn image_buffer_len(width: usize, height: usize) -> Option<usize> {
        Some(<P as Pixel>::CHANNEL_COUNT as usize)
            .and_then(|size| size.checked_mul(width as usize))
            .and_then(|size| size.checked_mul(height as usize))
    }

    #[inline(always)]
    fn pixel_indices(&self, x: usize, y: usize) -> Option<ops::Range<usize>> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(self.pixel_indices_unchecked(x, y))
    }

    #[inline(always)]
    fn pixel_indices_unchecked(&self, x: usize, y: usize) -> ops::Range<usize> {
        let channel_count = <P as Pixel>::CHANNEL_COUNT as usize;
        let min_index = (y * self.width + x) * channel_count;

        min_index..min_index + channel_count
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<&P> {
        if x >= self.width {
            return None;
        }
        let channel_count = <P as Pixel>::CHANNEL_COUNT as usize;
        let i = y
            .saturating_mul(self.width)
            .saturating_add(x)
            .saturating_mul(channel_count);

        self.data
            .get(i..i + channel_count)
            .map(|indices| <P as Pixel>::from_slice(indices))
    }

    pub fn get_pixel_unchecked(&self, x: usize, y: usize) -> &P {
        match self.pixel_indices(x, y) {
            None => panic!(
                "Index out of bounds: (width, height) is {:?}, but index is {:?}",
                (self.width, self.height),
                (x, y)
            ),
            Some(indices) => <P as Pixel>::from_slice(&self.data[indices]),
        }
    }

    pub fn as_ptr(&self) -> *const P {
        self.data.as_ptr() as *const P
    }
}

impl<P, Storage> TextureBuffer2D<P, Storage>
where
    P: Pixel,
    Storage: ops::Deref<Target = [P::Subpixel]> + ops::DerefMut,
{
    pub fn pixels_mut(&mut self) -> PixelsMut<P> {
        let len = Self::image_buffer_len(self.width, self.height).unwrap();
        let subpixels = &mut self.data[..len];

        PixelsMut {
            chunks: subpixels.chunks_exact_mut(<P as Pixel>::CHANNEL_COUNT as usize),
        }
    }

    pub fn enumerate_pixels_mut(&mut self) -> EnumeratePixelsMut<P> {
        let width = self.width;

        EnumeratePixelsMut {
            pixels: self.pixels_mut(),
            x: 0,
            y: 0,
            width,
        }
    }

    pub fn get_pixel_mut(&mut self, x: usize, y: usize) -> Option<&mut P> {
        if x >= self.width {
            return None;
        }
        let channel_count = <P as Pixel>::CHANNEL_COUNT as usize;
        let i = y
            .saturating_mul(self.width)
            .saturating_add(x)
            .saturating_mul(channel_count);

        self.data
            .get_mut(i..i + channel_count)
            .map(|indices| <P as Pixel>::from_slice_mut(indices))
    }

    pub fn get_pixel_mut_unchecked(&mut self, x: usize, y: usize) -> &mut P {
        match self.pixel_indices(x, y) {
            None => panic!(
                "Index out of bounds: (width, height) is {:?}, but index is {:?}",
                (self.width, self.height),
                (x, y)
            ),
            Some(indices) => <P as Pixel>::from_slice_mut(&mut self.data[indices]),
        }
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, pixel: P) {
        *self.get_pixel_mut_unchecked(x, y) = pixel;
    }

    pub fn flip_vertical(&mut self) {
        let height = self.height();
        let width = self.width_subpixels();
        let half_height = self.height() / 2;
        for row in 0..half_height {
            for col in 0..width {
                let temp = self.data[row * width + col];
                self.data[row * width + col] = self.data[((height - row - 1) * width) + col];
                self.data[((height - row - 1) * width) + col] = temp;
            }
        }
    }
}

impl<P> TextureBuffer2D<P, Vec<P::Subpixel>> 
where
    P: Pixel,
{
    pub fn new(width: usize, height: usize) -> TextureBuffer2D<P, Vec<P::Subpixel>> {
        let size = Self::image_buffer_len(width, height).unwrap();

        TextureBuffer2D { 
            width, 
            height, 
            data: vec![<P::Subpixel as Zero>::zero(); size],
            _marker: PhantomData,
        }
    }

    pub fn from_fill(width: usize, height: usize, fill_value: P) -> TextureBuffer2D<P, Vec<P::Subpixel>> {
        let mut buffer = TextureBuffer2D::<P, Vec<P::Subpixel>>::new(width, height);
        for pixel in buffer.pixels_mut() {
            *pixel = fill_value;
        }

        buffer
    }

    pub fn from_fn<Op>(width: usize, height: usize, mut op: Op) -> TextureBuffer2D<P, Vec<P::Subpixel>>
    where
        Op: FnMut(usize, usize) -> P
    {
        let mut buffer = TextureBuffer2D::<P, Vec<P::Subpixel>>::new(width, height);
        for (x, y, pixel) in buffer.enumerate_pixels_mut() {
            *pixel = op(x, y);
        }

        buffer
    }
}

impl<P, Storage> ops::Index<(usize, usize)> for TextureBuffer2D<P, Storage>
where
    P: Pixel,
    Storage: ops::Deref<Target = [P::Subpixel]>
{
    type Output = P;

    #[inline]
    fn index(&self, _index: (usize, usize)) -> &Self::Output {
        self.get_pixel_unchecked(_index.0, _index.1)
    }
}

impl<P, Storage> ops::IndexMut<(usize, usize)> for TextureBuffer2D<P, Storage> 
where
    P: Pixel,
    Storage: ops::Deref<Target = [P::Subpixel]> + ops::DerefMut,
{
    #[inline]
    fn index_mut(&mut self, _index: (usize, usize)) -> &mut Self::Output {
        self.get_pixel_mut_unchecked(_index.0, _index.1)
    }
}

impl<P, Storage> Default for TextureBuffer2D<P, Storage> 
where
    P: Pixel,
    Storage: Default,
{
    fn default() -> Self {
        TextureBuffer2D {
            width: 0,
            height: 0,
            data: Storage::default(),
            _marker: PhantomData,
        }
    }
}

