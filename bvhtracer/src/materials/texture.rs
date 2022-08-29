use image;
use image::codecs::jpeg::{
    JpegDecoder,
};
use image::codecs::png::{
    PngDecoder, 
};
use image::{
    ImageDecoder,
};

use std::io;

/*
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ColorType {
    L8,
    Rgb8,
    Rgba8,
}

#[derive(Clone, Debug)]
pub struct TextureImage2D {
    width: u32,
    height: u32,
    pub color_type: ColorType,
    pub bytes_per_pixel: u32,
    data: Vec<u8>,
}

impl TextureImage2D {
    pub fn new(width: u32, height: u32, color_type: ColorType, bytes_per_pixel: u32, data: Vec<u8>) -> Self {
        Self {
            width: width,
            height: height,
            color_type: color_type,
            bytes_per_pixel: bytes_per_pixel,
            data: data,
        }
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    pub fn width(&self) -> usize {
        self.width as usize
    } 

    pub fn height(&self) -> usize {
        self.height as usize
    }
}

impl Default for TextureImage2D {
    fn default() -> Self {
        Self { 
            width: 0, 
            height: 0, 
            color_type: ColorType::Rgb8, 
            bytes_per_pixel: 3, 
            data: vec![] 
        }
    }
}
*/

