use crate::texture_buffer::*;
use cglinalg::{
    Vector2,
};


pub trait Material: Clone {
    type P: Pixel;

    fn evaluate(&self, uv: Vector2<f32>) -> Self::P;
}

#[derive(Clone, Debug)]
pub struct TextureMaterial<P> {
    texture: TextureBuffer2D<P, Vec<u8>>,
}

impl<P> TextureMaterial<P>
where
    P: Pixel
{
    pub fn new(texture: TextureBuffer2D<P, Vec<u8>>) -> Self {
        Self { texture, }
    }
}

impl<P> Default for TextureMaterial<P>
where
    P: Pixel
{
    fn default() -> Self {
        Self { 
            texture: TextureBuffer2D::default(),
        }
    }
}

impl<P> Material for TextureMaterial<P>
where
    P: Pixel<Subpixel = u8>
{
    type P = P;

    fn evaluate(&self, uv: Vector2<f32>) -> Self::P {
        let width_f32 = self.texture.width() as f32;
        let height_f32 = self.texture.height() as f32;
        let iu = ((uv.x * width_f32) as usize) % self.texture.width();
        let iv = ((uv.y * height_f32) as usize) % self.texture.height();
        let texel = self.texture[(iu, iv)];

        texel
    }
}

