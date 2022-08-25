use crate::texture_buffer::*;
use crate::scene::*;
use crate::renderer::*;


pub trait AppState {
    fn update(&mut self, elapsed: f64);

    fn active_scene(&self) -> &Scene;

    fn active_scene_mut(&mut self) -> &mut Scene;
}

pub struct App {
    renderer_state: RendererState,
    state: Box<dyn AppState>,
    renderer: Renderer,
}
    
impl App {
   pub fn new(pixel_shader: Box<dyn PixelShader>, accumulator: Box<dyn Accumulator>, state: Box<dyn AppState>, renderer: Renderer, width: usize, height: usize) -> Self {
        let renderer_state = RendererState::new(
            accumulator,
            pixel_shader,
            width,
            height
        );
    
        Self { renderer_state, state, renderer, }
    }

    pub fn update(&mut self, elapsed: f64) {
        self.state.update(elapsed);
    }

    pub fn frame_buffer(&self) -> &FrameBuffer<Rgba<u8>> {
        self.renderer_state.frame_buffer()
    }

    pub fn frame_buffer_mut(&mut self) -> &mut FrameBuffer<Rgba<u8>> {
        self.renderer_state.frame_buffer_mut()
    }

    pub fn render(&mut self) -> usize {
        self.renderer.render(&mut self.renderer_state, self.state.active_scene())
    }
}

