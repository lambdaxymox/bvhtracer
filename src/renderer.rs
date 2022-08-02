use crate::frame_buffer::*;
use crate::scene::*;


pub struct Renderer {
}

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&mut self, scene: &Scene, frame_buffer: &mut FrameBuffer<Rgba<u8>, Vec<u8>>) -> usize {
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
                    let mut ray = scene.active_camera().get_ray_world(
                        (tile_width * x + u) as f32 / frame_buffer.width() as f32,
                        (tile_height * y + v) as f32 / frame_buffer.height() as f32,
                    );
                    if let Some(intersection) = scene.intersect(&ray) {
                        ray.t = intersection.ray.t;
                    }
                    let color = if ray.t < f32::MAX {
                        let _color = 255 - (((ray.t - 3_f32) * 80_f32) as i32) as u32;
                        let c = _color * 0x010101;
                        let r = ((c & 0x00FF0000) >> 16) as u8;
                        let g = ((c & 0x0000FF00) >> 8) as u8;
                        let b = (c & 0x000000FF) as u8;
                        
                        Rgba::new(r, g, b, 255)
                    } else {
                        Rgba::new(0, 0, 0, 255)
                    };

                    frame_buffer[(tile_height * x + u, tile_height * y + v)] = color;
                    rays_traced += 1;
                }
            }
        }

        rays_traced
    }
}