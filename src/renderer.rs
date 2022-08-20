use crate::texture_buffer::*;
use crate::materials::*;
use crate::scene::*;
use crate::query::{
    Ray,
};
use cglinalg::{
    Magnitude,
    Vector3,
};
use std::ops;


#[derive(Clone, Debug, PartialEq)]
pub struct FrameBuffer<P> 
where
    P: Pixel,
{
    radiometric: Vec<Vector3<f32>>,
    photometric: TextureBuffer2D<P, Vec<P::Subpixel>>,
}

impl<P> FrameBuffer<P> 
where
    P: Pixel,
{
    pub fn from_value(width: usize, height: usize, value: P) -> Self {
        let photometric = TextureBuffer2D::from_fill(width, height, value);
        
        Self {
            radiometric: vec![Vector3::zero(); width * height],
            photometric: photometric,
        }
    }

    #[inline]
    pub fn radiometric(&self) -> &[Vector3<f32>] {
        &self.radiometric
    }

    #[inline]
    pub fn radiometric_mut(&mut self) -> &mut [Vector3<f32>] {
        &mut self.radiometric
    }

    #[inline]
    pub fn photometric(&self) -> &TextureBuffer2D<P, Vec<P::Subpixel>> {
        &self.photometric
    }

    #[inline]
    pub fn photometric_mut(&mut self) -> &mut TextureBuffer2D<P, Vec<P::Subpixel>> {
        &mut self.photometric
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.photometric.width()
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.photometric.height()
    }

    #[inline]
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width(), self.height())
    }
}


pub trait RenderingPipeline {
    fn render(&mut self, scene: &Scene, frame_buffer: &mut FrameBuffer<Rgba<u8>>) -> usize;
}

pub struct DepthMappingPipeline {}

impl DepthMappingPipeline {
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderingPipeline for DepthMappingPipeline {
    fn render(&mut self, scene: &Scene, frame_buffer: &mut FrameBuffer<Rgba<u8>>) -> usize {
        let mut rays_traced = 0;
        let tile_width = 8;
        let tile_height = 8;
        let tile_count_x = 80;
        let tile_count_y = 80;
        let tile_count = tile_count_x * tile_count_y;
        for tile in 0..tile_count {
            let x = tile % tile_count_x;
            let y = tile / tile_count_y;
            for v in 0..tile_height {
                for u in 0..tile_width {
                    let ray = scene.active_camera().get_ray_world(
                        (tile_width * x + u) as f32 / frame_buffer.width() as f32,
                        (tile_height * y + v) as f32 / frame_buffer.height() as f32,
                    );
                    let nearest_t = if let Some(intersection) = scene.intersect(&ray) {
                        intersection.ray.t
                    } else {
                        ray.t
                    };
                    let color = if nearest_t < f32::MAX {
                        let _color = 255 - (((nearest_t - 3_f32) * 80_f32) as i32) as u32;
                        let c = _color * 0x010101;
                        let r = ((c & 0x00FF0000) >> 16) as u8;
                        let g = ((c & 0x0000FF00) >> 8) as u8;
                        let b = (c & 0x000000FF) as u8;
                        
                        Rgba::new(r, g, b, 255)
                    } else {
                        Rgba::new(0, 0, 0, 255)
                    };
                    frame_buffer.photometric_mut()[(tile_width * x + u, tile_height * y + v)] = color;
                    rays_traced += 1;
                }
            }
        }

        rays_traced
    }
}

pub struct UvMappingPipeline {}

impl UvMappingPipeline {
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderingPipeline for UvMappingPipeline {
    fn render(&mut self, scene: &Scene, frame_buffer: &mut FrameBuffer<Rgba<u8>>) -> usize {
        let mut rays_traced = 0;
        let tile_width = 8;
        let tile_height = 8;
        let tile_count_x = 80;
        let tile_count_y = 80;
        let tile_count = tile_count_x * tile_count_y;
        for tile in 0..tile_count {
            let x = tile % tile_count_x;
            let y = tile / tile_count_y;
            for v in 0..tile_height {
                for u in 0..tile_width {
                    let ray = scene.active_camera().get_ray_world(
                        (tile_width * x + u) as f32 / frame_buffer.width() as f32,
                        (tile_height * y + v) as f32 / frame_buffer.height() as f32,
                    );
                    let uv_coords = if let Some(intersection) = scene.intersect(&ray) {
                        Vector3::new(
                            intersection.interaction.u, 
                            intersection.interaction.v, 
                            1_f32 - (intersection.interaction.u + intersection.interaction.v)
                        )
                    } else {
                        Vector3::zero()
                    };
                    let color = {
                        let r = u8::min(255, (255_f32 * uv_coords.x) as u8);
                        let g = u8::min(255, (255_f32 * uv_coords.y) as u8);
                        let b = u8::min(255, (255_f32 * uv_coords.z) as u8);

                        Rgba::new(r, g, b, 255)
                    };
                    frame_buffer.photometric_mut()[(tile_width * x + u, tile_height * y + v)] = color;
                    rays_traced += 1;
                }
            }
        }

        rays_traced
    }
}

pub struct NormalMappingPipeline {}

impl NormalMappingPipeline {
    pub fn new() -> Self {
        Self {}
    }

    fn path_trace(&mut self, scene: &Scene, ray: &Ray<f32>) -> Vector3<f32> {
        if let Some(intersection) = scene.intersect(ray) {
            let primitive_index = intersection.instance_primitive.primitive_index();
            let instance_index = intersection.instance_primitive.instance_index();
            let primitive_normals = { 
                let model = scene.get_unchecked(instance_index as usize).model().model();
                let borrow = model.borrow();
                let normals = borrow.mesh().normals();
                normals[primitive_index as usize]
            };
            let normal = {
                let _normal_model_space = primitive_normals[1] * intersection.interaction.u +
                    primitive_normals[2] * intersection.interaction.v +
                    primitive_normals[0] * (1_f32 - (intersection.interaction.u + intersection.interaction.v));
                let object = scene.get_unchecked(instance_index as usize);
                let _normal_world_space = (object.get_transform() * _normal_model_space.extend(0_f32)).contract();
                let normalized = (_normal_world_space).normalize();
                (normalized + Vector3::from_fill(1_f32)) * 0.5
            };

            return normal;
        } else {
            return Vector3::zero();
        }
    }
}

impl RenderingPipeline for NormalMappingPipeline {
    fn render(&mut self, scene: &Scene, frame_buffer: &mut FrameBuffer<Rgba<u8>>) -> usize {
        let mut rays_traced = 0;
        let tile_width = 8;
        let tile_height = 8;
        let tile_count_x = 80;
        let tile_count_y = 80;
        let tile_count = tile_count_x * tile_count_y;
        for tile in 0..tile_count {
            let x = tile % tile_count_x;
            let y = tile / tile_count_y;
            for v in 0..tile_height {
                for u in 0..tile_width {
                    let ray = scene.active_camera().get_ray_world(
                        (tile_width * x + u) as f32 / frame_buffer.width() as f32,
                        (tile_height * y + v) as f32 / frame_buffer.height() as f32,
                    );
                    let normal = self.path_trace(scene, &ray);
                    let color = {
                        let r = u8::min(255, (255_f32 * normal.x) as u8);
                        let g = u8::min(255, (255_f32 * normal.y) as u8);
                        let b = u8::min(255, (255_f32 * normal.z) as u8);

                        Rgba::new(r, g, b, 255)
                    };
                    frame_buffer.photometric_mut()[(tile_width * x + u, tile_height * y + v)] = color;
                    rays_traced += 1;
                }
            }
        }

        rays_traced
    }
}

pub struct PathTracer {}

impl PathTracer {
    pub fn new() -> Self {
        Self {}
    }

    fn path_trace(&mut self, scene: &Scene, ray: &Ray<f32>) -> Vector3<f32> {
        fn rgb8_to_rgb32f(texel: Rgb<u8>) -> Vector3<f32> {
            let s = 1_f32 / 256_f32;
            let r = texel.r() as f32;
            let g = texel.g() as f32;
            let b = texel.b() as f32;

            Vector3::new(r * s, g * s, b * s)
        }

        if let Some(intersection) = scene.intersect(ray) {
            let primitive_index = intersection.instance_primitive.primitive_index();
            let instance_index = intersection.instance_primitive.instance_index();
            let primitive_tex_coords = { 
                let model = scene.get_unchecked(instance_index as usize).model().model();
                let borrow = model.borrow();
                let tex_coords = borrow.mesh().tex_coords();
                tex_coords[primitive_index as usize]
            };
            let uv_coords = primitive_tex_coords[1] * intersection.interaction.u +
                primitive_tex_coords[2] * intersection.interaction.v +
                primitive_tex_coords[0] * (1_f32 - (intersection.interaction.u + intersection.interaction.v));
            let texel = {
                let model =  scene.get_unchecked(instance_index as usize).model().model();
                let borrow = model.borrow();
                let material = borrow.texture();
                material.evaluate(uv_coords)
            };

            return rgb8_to_rgb32f(texel);
        } else {
            return Vector3::zero();
        }
    }
}

impl RenderingPipeline for PathTracer {
    // fn render(&mut self, scene: &Scene, frame_buffer: &mut TextureBuffer2D<Rgba<u8>, Vec<u8>>) -> usize {
    fn render(&mut self, scene: &Scene, frame_buffer: &mut FrameBuffer<Rgba<u8>>) -> usize {
        let mut rays_traced = 0;
        let tile_width = 8;
        let tile_height = 8;
        let tile_count_x = 80;
        let tile_count_y = 80;
        let tile_count = tile_count_x * tile_count_y;
        for tile in 0..tile_count {
            let x = tile % tile_count_x;
            let y = tile / tile_count_y;
            for v in 0..tile_height {
                for u in 0..tile_width {
                    let ray = scene.active_camera().get_ray_world(
                        (tile_width * x + u) as f32 / frame_buffer.width() as f32,
                        (tile_height * y + v) as f32 / frame_buffer.height() as f32,
                    );
                    let pixel_address = (x * tile_width + u) + (y * tile_height + v) * frame_buffer.width();
                    let pixel = self.path_trace(scene, &ray);
                    frame_buffer.radiometric_mut()[pixel_address] = pixel;
                    rays_traced += 1;
                }
            }
        }

        for tile in 0..tile_count {
            let x = tile % tile_count_x;
            let y = tile / tile_count_y;
            for v in 0..tile_height {
                for u in 0..tile_width {
                    let pixel_address = (x * tile_width + u) + (y * tile_height + v) * frame_buffer.width();
                    let pixel = frame_buffer.radiometric()[pixel_address];
                    let color = {
                        let r = u8::min(255, (255_f32 * pixel.x) as u8);
                        let g = u8::min(255, (255_f32 * pixel.y) as u8);
                        let b = u8::min(255, (255_f32 * pixel.z) as u8);

                        Rgba::new(r, g, b, 255)
                    };
                    frame_buffer.photometric_mut()[(x * tile_width + u, y * tile_height + v)] = color;
                }
            }
        }

        rays_traced
    }
}

pub struct Renderer {
    pipeline: Box<dyn RenderingPipeline>,
}

impl Renderer {
    pub fn new(pipeline: Box<dyn RenderingPipeline>) -> Self {
        Self { pipeline, }
    }

    pub fn render(&mut self, scene: &Scene, frame_buffer: &mut FrameBuffer<Rgba<u8>>) -> usize {
        self.pipeline.render(scene, frame_buffer)
    }
}

