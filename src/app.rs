use crate::texture_buffer::*;
use crate::scene::*;
use crate::renderer::*;


pub trait AppState {
    fn update(&mut self, elapsed: f64);

    fn active_scene(&self) -> &Scene;

    fn active_scene_mut(&mut self) -> &mut Scene;
}

pub struct App {
    pixel_shader: Box<dyn PixelShader>,
    accumulator: Box<dyn Accumulator>,
    accumulation_buffer: AccumulationBuffer<f32>,
    pub frame_buffer: FrameBuffer<Rgba<u8>>,
    state: Box<dyn AppState>,
    renderer: Renderer,
}
    
impl App {
   pub fn new(pixel_shader: Box<dyn PixelShader>, accumulator: Box<dyn Accumulator>, state: Box<dyn AppState>, renderer: Renderer, width: usize, height: usize) -> Self {
        let accumulation_buffer = AccumulationBuffer::new(width, height);
        let frame_buffer = FrameBuffer::from_fill(
            width, 
            height,
            Rgba::from([0, 0, 0, 255])
        );
    
        Self { pixel_shader, accumulator, accumulation_buffer, frame_buffer, state, renderer, }
    }

    pub fn update(&mut self, elapsed: f64) {
        self.state.update(elapsed);
    }

    pub fn render(&mut self) -> usize {
        self.renderer.render(self.state.active_scene(), &mut *self.accumulator, &*self.pixel_shader, &mut self.accumulation_buffer, &mut self.frame_buffer)
    }
}

