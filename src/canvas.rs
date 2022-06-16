use std::ops;


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    #[inline]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Rgba {
        Rgba { 
            r, b, g, a,
        }
    }

    #[inline]
    pub const fn zero() -> Rgba {
        Rgba { r: 0, g: 0, b: 0, a: 1 }
    }
}

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Rgba>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self { 
            width, height, data: vec![Rgba::zero(); width * height]
        }
    }

    pub fn clear(&mut self) {
        let zero = Rgba::zero();
        for pixel in self.data.as_mut_slice() {
            *pixel = zero;
        }
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

