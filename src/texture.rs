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


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ColorType {
    L8,
    Rgb8,
    Rgba8,
}

#[derive(Clone, Debug)]
pub struct TextureImage2D {
    pub width: u32,
    pub height: u32,
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


pub fn from_png_buffer(buffer: &[u8]) -> TextureImage2D {
    let cursor = io::Cursor::new(buffer);
    let image_decoder = PngDecoder::new(cursor).unwrap();
    let (width, height) = image_decoder.dimensions();
    let total_bytes = image_decoder.total_bytes();
    let image_decoder_color_type = image_decoder.color_type();
    let color_type = match image_decoder_color_type {
        image::ColorType::Rgb8 => ColorType::Rgb8,
        image::ColorType::Rgba8 => ColorType::Rgba8,
        _ => ColorType::L8,
    };
    let bytes_per_pixel = image_decoder_color_type.bytes_per_pixel() as u32;
    let mut image_data = vec![0 as u8; total_bytes as usize];
    image_decoder.read_image(&mut image_data).unwrap();

    assert_eq!(total_bytes, (width * height * bytes_per_pixel) as u64);

    TextureImage2D::new(width, height, color_type, bytes_per_pixel, image_data)
}

pub fn from_png_reader<R: io::Read>(reader: &mut R) -> TextureImage2D {
    let image_decoder = PngDecoder::new(reader).unwrap();
    let (width, height) = image_decoder.dimensions();
    let total_bytes = image_decoder.total_bytes();
    let image_decoder_color_type = image_decoder.color_type();
    let color_type = match image_decoder_color_type {
        image::ColorType::Rgb8 => ColorType::Rgb8,
        image::ColorType::Rgba8 => ColorType::Rgba8,
        _ => ColorType::L8,
    };
    let bytes_per_pixel = image_decoder_color_type.bytes_per_pixel() as u32;
    let mut image_data = vec![0 as u8; total_bytes as usize];
    image_decoder.read_image(&mut image_data).unwrap();

    assert_eq!(total_bytes, (width * height * bytes_per_pixel) as u64);

    TextureImage2D::new(width, height, color_type, bytes_per_pixel, image_data)
}

pub fn from_jpeg_reader<R: io::Read>(reader: &mut R) -> TextureImage2D {
    let image_decoder = JpegDecoder::new(reader).unwrap();
    let (width, height) = image_decoder.dimensions();
    let total_bytes = image_decoder.total_bytes();
    let image_decoder_color_type = image_decoder.color_type();
    let color_type = match image_decoder_color_type {
        image::ColorType::Rgb8 => ColorType::Rgb8,
        image::ColorType::Rgba8 => ColorType::Rgba8,
        _ => ColorType::L8,
    };
    let bytes_per_pixel = image_decoder_color_type.bytes_per_pixel() as u32;
    let mut image_data = vec![0 as u8; total_bytes as usize];
    image_decoder.read_image(&mut image_data).unwrap();

    assert_eq!(total_bytes, (width * height * bytes_per_pixel) as u64);

    TextureImage2D::new(width, height, color_type, bytes_per_pixel, image_data)
}

