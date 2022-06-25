use crate::pixel::*;

use std::ops;


pub struct ImageBuffer<P> {
    width: usize,
    height: usize,
    pub data: Vec<P>,
}

impl<P> ImageBuffer<P>
where
    P: Pixel,
{
    pub fn from_fn<Op>(width: usize, height: usize, mut op: Op) -> Self
    where
        Op: FnMut(usize, usize) -> P
    {
        let mut data = vec![];
        for x in 0..width {
            for y in 0..height {
                data.push(op(x, y));
            }
        }

        Self { width, height, data, }
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
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<&P> {
        if x < self.width && y < self.height {
            return Some(self.get_pixel_unchecked(x, y));
        }

        None
    }

    pub fn get_pixel_unchecked(&self, x: usize, y: usize) -> &P {
        let row = y * self.width;
            
        &self.data[row + x]
    }

    pub fn get_pixel_mut(&mut self, x: usize, y: usize) -> Option<&mut P> {
        if x < self.width && y < self.height {
            return Some(self.get_pixel_mut_unchecked(x, y));
        }

        None
    }

    pub fn get_pixel_mut_unchecked(&mut self, x: usize, y: usize) -> &mut P {
        let row = y * self.width;
            
        &mut self.data[row + x]
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, pixel: P) {
        *self.get_pixel_mut_unchecked(x, y) = pixel;
    }

    pub fn as_ptr(&self) -> *const P {
        self.data.as_ptr()
    }
}

impl<P> ops::Index<usize> for ImageBuffer<P>
where
    P: Pixel,
{
    type Output = [P];

    #[inline]
    fn index(&self, _index: usize) -> &Self::Output {
        let row_start = _index * self.width;
        let row_end = row_start + self.width;

        &self.data[row_start..row_end]
    }
}

impl<P> ops::IndexMut<usize> for ImageBuffer<P> 
where
    P: Pixel,
{
    #[inline]
    fn index_mut(&mut self, _index: usize) -> &mut Self::Output {
        let row_start = _index * self.width;
        let row_end = row_start + self.width;

        &mut self.data[row_start..row_end]
    }
}

