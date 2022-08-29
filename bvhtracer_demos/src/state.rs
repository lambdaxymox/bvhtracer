extern crate bvhtracer;
extern crate cglinalg;


#[allow(clippy::all)]
mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}


use bvhtracer::*;
use crate::app::*;
use crate::context::*;
use cglinalg::{
    Matrix4x4,
    Vector3,
    Radians,
};
use std::io;


impl GpuTextureBuffer2D for TextureBuffer2D<Rgba<u8>, Vec<u8>> {
    fn format(&self) -> GpuTextureFormat { 
        GpuTextureFormat::Rgba8
    }

    fn width(&self) -> usize {
        TextureBuffer2D::width(self)
    }

    fn height(&self) -> usize {
        TextureBuffer2D::height(self)
    }

    fn dimensions(&self) -> (usize, usize) {
        TextureBuffer2D::dimensions(self)
    }

    fn as_ptr(&self) -> *const u8 {
        TextureBuffer2D::as_ptr(self) as *const u8
    }

    fn as_mut_ptr(&mut self) -> *mut u8 {
        TextureBuffer2D::as_mut_ptr(self) as *mut u8
    }
}


