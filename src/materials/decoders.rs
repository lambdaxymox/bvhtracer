use crate::texture_buffer::*;


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

pub enum TextureBufferParsed {
    Rgba8(TextureBuffer2D<Rgba<u8>, Vec<u8>>),
    Rgb8(TextureBuffer2D<Rgb<u8>, Vec<u8>>),
}

pub fn from_png_buffer(buffer: &[u8]) -> Option<TextureBufferParsed> {
    let cursor = io::Cursor::new(buffer);
    let image_decoder = PngDecoder::new(cursor).unwrap();
    let (width, height) = image_decoder.dimensions();
    let total_bytes = image_decoder.total_bytes();
    let image_decoder_color_type = image_decoder.color_type();
    match image_decoder_color_type {
        image::ColorType::Rgb8 => {
            let bytes_per_pixel = image_decoder_color_type.bytes_per_pixel() as u32;
            let mut buffer = vec![0 as u8; total_bytes as usize];
            image_decoder.read_image(&mut buffer).unwrap();

            assert_eq!(total_bytes, (width * height * bytes_per_pixel) as u64);

            let texture = TextureBuffer2D::from_raw(
                    width as usize, 
                    height as usize, 
                    buffer
                ).unwrap();
            
            Some(TextureBufferParsed::Rgb8(texture))
        }
        image::ColorType::Rgba8 => {
            let bytes_per_pixel = image_decoder_color_type.bytes_per_pixel() as u32;
            let mut buffer = vec![0 as u8; total_bytes as usize];
            image_decoder.read_image(&mut buffer).unwrap();

            assert_eq!(total_bytes, (width * height * bytes_per_pixel) as u64);

            let texture = TextureBuffer2D::from_raw(
                    width as usize, 
                    height as usize, 
                    buffer
                ).unwrap();
            
            Some(TextureBufferParsed::Rgba8(texture))
        }
        _ => return None,
    }
}

pub fn from_png_reader<R: io::Read>(reader: &mut R) -> Option<TextureBufferParsed> {
    let image_decoder = PngDecoder::new(reader).unwrap();
    let (width, height) = image_decoder.dimensions();
    let total_bytes = image_decoder.total_bytes();
    let image_decoder_color_type = image_decoder.color_type();
    match image_decoder_color_type {
        image::ColorType::Rgb8 => {
            let bytes_per_pixel = image_decoder_color_type.bytes_per_pixel() as u32;
            let mut buffer = vec![0 as u8; total_bytes as usize];
            image_decoder.read_image(&mut buffer).unwrap();

            assert_eq!(total_bytes, (width * height * bytes_per_pixel) as u64);

            let texture = TextureBuffer2D::from_raw(
                    width as usize, 
                    height as usize, 
                    buffer
                ).unwrap();
            
            Some(TextureBufferParsed::Rgb8(texture))
        }
        image::ColorType::Rgba8 => {
            let bytes_per_pixel = image_decoder_color_type.bytes_per_pixel() as u32;
            let mut buffer = vec![0 as u8; total_bytes as usize];
            image_decoder.read_image(&mut buffer).unwrap();

            assert_eq!(total_bytes, (width * height * bytes_per_pixel) as u64);

            let texture = TextureBuffer2D::from_raw(
                    width as usize, 
                    height as usize, 
                    buffer
                ).unwrap();
            
            Some(TextureBufferParsed::Rgba8(texture))
        }
        _ => return None,
    }
}

pub fn from_jpeg_reader<R: io::Read>(reader: &mut R) -> Option<TextureBufferParsed> {
    let image_decoder = JpegDecoder::new(reader).unwrap();
    let (width, height) = image_decoder.dimensions();
    let total_bytes = image_decoder.total_bytes();
    let image_decoder_color_type = image_decoder.color_type();
    match image_decoder_color_type {
        image::ColorType::Rgb8 => {
            let bytes_per_pixel = image_decoder_color_type.bytes_per_pixel() as u32;
            let mut buffer = vec![0 as u8; total_bytes as usize];
            image_decoder.read_image(&mut buffer).unwrap();

            assert_eq!(total_bytes, (width * height * bytes_per_pixel) as u64);

            let texture = TextureBuffer2D::from_raw(
                    width as usize, 
                    height as usize, 
                    buffer
                ).unwrap();
            
            Some(TextureBufferParsed::Rgb8(texture))
        }
        image::ColorType::Rgba8 => {
            let bytes_per_pixel = image_decoder_color_type.bytes_per_pixel() as u32;
            let mut buffer = vec![0 as u8; total_bytes as usize];
            image_decoder.read_image(&mut buffer).unwrap();

            assert_eq!(total_bytes, (width * height * bytes_per_pixel) as u64);

            let texture = TextureBuffer2D::from_raw(
                    width as usize, 
                    height as usize, 
                    buffer
                ).unwrap();
            
            Some(TextureBufferParsed::Rgba8(texture))
        }
        _ => return None,
    }
}

