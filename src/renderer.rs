use crate::texture_buffer::*;
use crate::materials::*;
use crate::scene::*;
use crate::query::{
    Ray,
};
use cglinalg::{
    Magnitude,
    SimdScalarFloat,
    Vector3,
};
use std::ops;


#[derive(Clone, Debug, PartialEq)]
pub struct AccumulationBuffer<S> {
    data: Vec<Vector3<S>>,
}

impl<S> AccumulationBuffer<S>
where
    S: SimdScalarFloat,
{
    pub fn new(width: usize, height: usize) -> Self {
        Self::from_fill(width, height, Vector3::zero())
    }

    pub fn from_fill(width: usize, height: usize, value: Vector3<S>) -> Self {
        Self {
            data: vec![value; width * height],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FrameBuffer<P> 
where
    P: Pixel,
{
    data: TextureBuffer2D<P, Vec<P::Subpixel>>,
}

impl<P> FrameBuffer<P> 
where
    P: Pixel,
{
    pub fn from_fill(width: usize, height: usize, value: P) -> Self {
        Self {
            data: TextureBuffer2D::from_fill(width, height, value),
        }
    }

    #[inline]
    pub fn photometric(&self) -> &TextureBuffer2D<P, Vec<P::Subpixel>> {
        &self.data
    }

    #[inline]
    pub fn photometric_mut(&mut self) -> &mut TextureBuffer2D<P, Vec<P::Subpixel>> {
        &mut self.data
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.data.width()
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.data.height()
    }

    #[inline]
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width(), self.height())
    }
}

pub trait Accumulator {
    fn evaluate(&mut self, scene: &Scene, ray: &Ray<f32>) -> Vector3<f32>;
}

pub trait PixelShader {
    fn evaluate(&self, accumulation_buffer: &mut AccumulationBuffer<f32>, radiance: &Vector3<f32>) -> Rgba<u8>;
}


pub struct IntersectionAccumulator {
    hit_value: Vector3<f32>,
    miss_value: Vector3<f32>,
}

impl IntersectionAccumulator {
    pub fn new(hit_value: Vector3<f32>, miss_value: Vector3<f32>) -> Self {
        Self { hit_value, miss_value, }
    }
}

impl Accumulator for IntersectionAccumulator {
    fn evaluate(&mut self, scene: &Scene, ray: &Ray<f32>) -> Vector3<f32> {
        if let Some(_) = scene.intersect(&ray) {
            self.hit_value
        } else {
            self.miss_value
        }
    }
}

pub struct IntersectionShader {
    hit_value: Rgba<u8>,
    miss_value: Rgba<u8>,
}

impl IntersectionShader {
    pub fn new(hit_value: Rgba<u8>, miss_value: Rgba<u8>) -> Self {
        Self { hit_value, miss_value, }
    }
}

impl PixelShader for IntersectionShader {
    fn evaluate(&self, accumulation_buffer: &mut AccumulationBuffer<f32>, radiance: &Vector3<f32>) -> Rgba<u8> {
        if !radiance.is_zero() {
            self.hit_value
        } else {
            self.miss_value
        }
    }
}

pub trait Integrator {
    fn evaluate(&mut self, scene: &Scene, accumulator: &mut dyn Accumulator, pixel_shader: &dyn PixelShader, accumulation_buffer: &mut AccumulationBuffer<f32>, frame_buffer: &mut FrameBuffer<Rgba<u8>>) -> usize;
}

pub struct DepthAccumulator {}

impl DepthAccumulator {
    pub fn new() -> Self {
        Self {}
    }
}

impl Accumulator for DepthAccumulator {
    fn evaluate(&mut self, scene: &Scene, ray: &Ray<f32>) -> Vector3<f32> {
        let nearest_t = if let Some(intersection) = scene.intersect(&ray) {
            intersection.interaction.t
        } else {
            f32::MAX
        };

        Vector3::from_fill(nearest_t)
    }
}

pub struct DepthMappingShader {
    scale: f32,
    offset: f32,
}

impl DepthMappingShader {
    pub fn new(scale: f32, offset: f32) -> Self {
        Self { scale, offset, }
    }
}

impl PixelShader for DepthMappingShader {
    fn evaluate(&self, accumulation_buffer: &mut AccumulationBuffer<f32>, radiance: &Vector3<f32>) -> Rgba<u8> {
        let nearest_t = radiance.x;
        if nearest_t < f32::MAX {
            let _color = 255 - (((nearest_t - self.offset) * self.scale) as i32) as u32;
            let c = _color * 0x010101;
            let r = ((c & 0x00FF0000) >> 16) as u8;
            let g = ((c & 0x0000FF00) >> 8) as u8;
            let b = (c & 0x000000FF) as u8;
            
            Rgba::new(r, g, b, 255)
        } else {
            Rgba::new(0, 0, 0, 255)
        }
    }
}


pub struct UvMappingAccumulator {}

impl UvMappingAccumulator {
    pub fn new() -> Self {
        Self {}
    }
}

impl Accumulator for UvMappingAccumulator {
    fn evaluate(&mut self, scene: &Scene, ray: &Ray<f32>) -> Vector3<f32> {
        if let Some(intersection) = scene.intersect(&ray) {
            Vector3::new(
                intersection.interaction.u, 
                intersection.interaction.v, 
                1_f32 - (intersection.interaction.u + intersection.interaction.v)
            )
        } else {
            Vector3::zero()
        }
    }
}

pub struct UvMappingShader {}

impl UvMappingShader {
    pub fn new() -> Self {
        Self {}
    }
}

impl PixelShader for UvMappingShader {
    fn evaluate(&self, accumulation_buffer: &mut AccumulationBuffer<f32>, radiance: &Vector3<f32>) -> Rgba<u8> {
        let r = u8::min(255, (255_f32 * radiance.x) as u8);
        let g = u8::min(255, (255_f32 * radiance.y) as u8);
        let b = u8::min(255, (255_f32 * radiance.z) as u8);

        Rgba::new(r, g, b, 255)
    }
}


pub struct NormalMappingAccumulator {}

impl NormalMappingAccumulator {
    pub fn new() -> Self {
        Self {}
    }
}

impl Accumulator for NormalMappingAccumulator {
    fn evaluate(&mut self, scene: &Scene, ray: &Ray<f32>) -> Vector3<f32> {
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

            normal
        } else {
            Vector3::zero()
        }
    }
}

pub struct NormalMappingShader {}

impl NormalMappingShader {
    pub fn new() -> Self {
        Self {}
    }
}

impl PixelShader for NormalMappingShader {
    fn evaluate(&self, accumulation_buffer: &mut AccumulationBuffer<f32>, radiance: &Vector3<f32>) -> Rgba<u8> {
        let r = u8::min(255, (255_f32 * radiance.x) as u8);
        let g = u8::min(255, (255_f32 * radiance.y) as u8);
        let b = u8::min(255, (255_f32 * radiance.z) as u8);

        Rgba::new(r, g, b, 255)
    }
}


pub struct TextureMaterialAccumulator {}

impl TextureMaterialAccumulator {
    pub fn new() -> Self {
        Self {}
    }
}

impl Accumulator for TextureMaterialAccumulator {
    fn evaluate(&mut self, scene: &Scene, ray: &Ray<f32>) -> Vector3<f32> {
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
                let model = scene.get_unchecked(instance_index as usize).model().model();
                let borrow = model.borrow();
                let material = borrow.texture();
                material.evaluate(uv_coords)
            };

            rgb8_to_rgb32f(texel)
        } else {
            Vector3::zero()
        }
    }
}

pub struct TextureMaterialShader {}

impl TextureMaterialShader {
    pub fn new() -> Self {
        Self {}
    }
}

impl PixelShader for TextureMaterialShader {
    fn evaluate(&self, accumulation_buffer: &mut AccumulationBuffer<f32>, radiance: &Vector3<f32>) -> Rgba<u8> {
        let r = u8::min(255, (255_f32 * radiance.x) as u8);
        let g = u8::min(255, (255_f32 * radiance.y) as u8);
        let b = u8::min(255, (255_f32 * radiance.z) as u8);

        Rgba::new(r, g, b, 255)
    }
}

pub struct PathTracer {}

impl PathTracer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Integrator for PathTracer {
    fn evaluate(&mut self, scene: &Scene, accumulator: &mut dyn Accumulator, pixel_shader: &dyn PixelShader, accumulation_buffer: &mut AccumulationBuffer<f32>, frame_buffer: &mut FrameBuffer<Rgba<u8>>) -> usize {
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
                    let radiance = accumulator.evaluate(scene, &ray);
                    accumulation_buffer.data[pixel_address] = radiance;
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
                    let radiance = accumulation_buffer.data[pixel_address];
                    let color = pixel_shader.evaluate(accumulation_buffer, &radiance);
                    frame_buffer.data[(x * tile_width + u, y * tile_height + v)] = color;
                }
            }
        }

        rays_traced
    }
}


pub struct Renderer {
    integrator: Box<dyn Integrator>,
}

impl Renderer {
    pub fn new(pipeline: Box<dyn Integrator>) -> Self {
        Self { integrator: pipeline, }
    }

    pub fn render(&mut self, scene: &Scene, accumulator: &mut dyn Accumulator, pixel_shader: &dyn PixelShader, accumulation_buffer: &mut AccumulationBuffer<f32>, frame_buffer: &mut FrameBuffer<Rgba<u8>>) -> usize {
        self.integrator.evaluate(scene, accumulator, pixel_shader, accumulation_buffer, frame_buffer)
    }
}

