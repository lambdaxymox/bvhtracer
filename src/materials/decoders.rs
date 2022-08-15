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
use std::marker::{
    PhantomData,
};


#[derive(Clone, Debug)]
pub enum TextureBufferError {
    ColorSpaceMismatch(),
}

pub type TextureBufferResult<T> = Result<T, TextureBufferError>;


pub trait TextureBufferDecoder<'a>: Sized {
    type Reader: Read + 'a;
    type Output;

    fn read_texture(self) -> TextureBufferResult<Self::Output>;
}

#[derive(Clone, Debug)]
pub struct PngTextureBufferDecoder<P, R> {
    reader: R,
    _marker: PhantomData<P>,
}

impl<P, R> PngTextureBufferDecoder<P, R>
where
    R: Read,
    P: Pixel,
{
    pub fn new(reader: R) -> Self {
        Self { 
            reader, 
            _marker: PhantomData,
        }
    }
}

impl<'a, R> TextureBufferDecoder<'a> for PngTextureBufferDecoder<Rgb<u8>, R> 
where
    R: Read + 'a,
{
    type Reader = R;
    type Output = TextureBuffer2D<Rgb<u8>, Vec<u8>>;

    fn read_texture(self) -> TextureBufferResult<Self::Output> {
        let image_decoder = PngDecoder::new(self.reader).unwrap();
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
                
                Ok(texture)
            }
            _ => Err(TextureBufferError::ColorSpaceMismatch())
        }
    }
}

impl<'a, R> TextureBufferDecoder<'a> for PngTextureBufferDecoder<Rgba<u8>, R> 
where
    R: Read + 'a,
{
    type Reader = R;
    type Output = TextureBuffer2D<Rgba<u8>, Vec<u8>>;

    fn read_texture(self) -> TextureBufferResult<Self::Output> {
        let image_decoder = PngDecoder::new(self.reader).unwrap();
        let (width, height) = image_decoder.dimensions();
        let total_bytes = image_decoder.total_bytes();
        let image_decoder_color_type = image_decoder.color_type();
        match image_decoder_color_type {
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
                
                Ok(texture)
            }
            _ => Err(TextureBufferError::ColorSpaceMismatch())
        }
    }
}

#[derive(Clone, Debug)]
pub struct JpegTextureBufferDecoder<P, R> {
    reader: R,
    _marker: PhantomData<P>,
}

impl<P, R> JpegTextureBufferDecoder<P, R>
where
    R: Read,
    P: Pixel,
{
    pub fn new(reader: R) -> Self {
        Self { 
            reader, 
            _marker: PhantomData,
        }
    }
}

impl<'a, R> TextureBufferDecoder<'a> for JpegTextureBufferDecoder<Rgb<u8>, R> 
where
    R: Read + 'a,
{
    type Reader = R;
    type Output = TextureBuffer2D<Rgb<u8>, Vec<u8>>;

    fn read_texture(self) -> TextureBufferResult<Self::Output> {
        let image_decoder = JpegDecoder::new(self.reader).unwrap();
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
                
                Ok(texture)
            }
            _ => Err(TextureBufferError::ColorSpaceMismatch()),
        }
    }
}

impl<'a, R> TextureBufferDecoder<'a> for JpegTextureBufferDecoder<Rgba<u8>, R> 
where
    R: Read + 'a,
{
    type Reader = R;
    type Output = TextureBuffer2D<Rgba<u8>, Vec<u8>>;

    fn read_texture(self) -> TextureBufferResult<Self::Output> {
        let image_decoder = JpegDecoder::new(self.reader).unwrap();
        let (width, height) = image_decoder.dimensions();
        let total_bytes = image_decoder.total_bytes();
        let image_decoder_color_type = image_decoder.color_type();
        match image_decoder_color_type {
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
                
                Ok(texture)
            }
            _ => Err(TextureBufferError::ColorSpaceMismatch())
        }
    }
}

