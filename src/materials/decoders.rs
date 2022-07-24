use crate::materials::*;


use image::{
    ImageDecoder,
};
use image::codecs::{
    png::PngDecoder,
    jpeg::JpegDecoder,
};

use std::fs::{
    File,
};
use std::io;
use std::io::{
    Read,
};
use std::path::{
    Path,
};


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

