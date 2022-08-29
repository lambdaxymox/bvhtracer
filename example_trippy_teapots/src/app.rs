use crate::gl;
use crate::context::*;
use bvhtracer::*;


use cglinalg::{
    Matrix4x4,
};
use glfw::{
    Action, 
    Key
};
use gl::types::{
    GLfloat,
};
use std::mem;
use std::ptr;
use std::ffi::{
    c_void,
};
use std::time::{
    SystemTime,
};


pub trait AppState {
    fn update(&mut self, elapsed: f64);

    fn active_scene(&self) -> &Scene;

    fn active_scene_mut(&mut self) -> &mut Scene;
}

pub struct App {
    context: GlContext,
    renderer_state: RendererState,
    state: Box<dyn AppState>,
    renderer: Renderer,
}
    
impl App {
   pub fn new(context: GlContext, pixel_shader: Box<dyn PixelShader>, accumulator: Box<dyn Accumulator>, state: Box<dyn AppState>, renderer: Renderer, width: usize, height: usize) -> Self {
        let renderer_state = RendererState::new(
            accumulator,
            pixel_shader,
            width,
            height
        );
    
        Self { context, renderer_state, state, renderer, }
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

    pub fn run(&mut self) {
        println!("Rendering scene.");
        let now = SystemTime::now();
        self.render();
        let elapsed = now.elapsed().unwrap();
        println!("Rendering time = {} s", elapsed.as_secs_f64());
    
        self.frame_buffer_mut().flip_vertical();
    
        let vertices: Vec<[f32; 2]> = vec![
            [1.0, 1.0], [-1.0,  1.0], [-1.0, -1.0],
            [1.0, 1.0], [-1.0, -1.0], [ 1.0, -1.0], 
        ];
        let tex_coords: Vec<[f32; 2]> = vec![
            [1.0, 1.0], [0.0, 1.0], [0.0, 0.0],
            [1.0, 1.0], [0.0, 0.0], [1.0, 0.0],
        ];
    
        let trans_mat = Matrix4x4::identity();
        let mut gui_scale_mat = Matrix4x4::identity();
    
        let shader_program = {
            let vertex_shader_source = include_str!("../shaders/shaders.vert.glsl");
            let fragment_shader_source = include_str!("../shaders/shaders.frag.glsl");
            let shader_source = ShaderSource::new(
                vertex_shader_source, 
                "shaders.vert.glsl", 
                fragment_shader_source, 
                "shaders.frag.glsl"
            );
            self.context.compile_shader(&shader_source).unwrap()
        };
       
        let vao = unsafe {
            gl::UseProgram(shader_program);
    
            let m_trans_location = gl::GetUniformLocation(shader_program, gl_str("m_trans").as_ptr());
            debug_assert!(m_trans_location > -1);
            let m_gui_scale_location = gl::GetUniformLocation(shader_program, gl_str("m_gui_scale").as_ptr());
            debug_assert!(m_gui_scale_location > -1);
    
            gl::UniformMatrix4fv(m_trans_location, 1, gl::FALSE, trans_mat.as_ptr());
            gl::UniformMatrix4fv(m_gui_scale_location, 1, gl::FALSE, gui_scale_mat.as_ptr());
    
            let vertices_len_bytes = (vertices.len() * mem::size_of::<[f32; 2]>()) as isize;
            let tex_coords_len_bytes = (tex_coords.len() * mem::size_of::<[f32; 2]>()) as isize;
    
            let v_pos_location = gl::GetAttribLocation(shader_program, gl_str("v_pos").as_ptr());
            debug_assert!(v_pos_location > -1);
            let v_pos_location = v_pos_location as u32;
            
            let v_tex_location = gl::GetAttribLocation(shader_program, gl_str("v_tex").as_ptr());
            debug_assert!(v_tex_location > -1);
            let v_tex_location = v_tex_location as u32;
    
            let mut vertex_vbo = 0;
            gl::GenBuffers(1, &mut vertex_vbo);
            debug_assert!(vertex_vbo > 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_vbo);
            gl::BufferData(gl::ARRAY_BUFFER, vertices_len_bytes, vertices.as_ptr() as *const c_void, gl::STATIC_DRAW);
    
            let mut tex_coords_vbo = 0;
            gl::GenBuffers(1, &mut tex_coords_vbo);
            debug_assert!(tex_coords_vbo > 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, tex_coords_vbo);
            gl::BufferData(gl::ARRAY_BUFFER, tex_coords_len_bytes, tex_coords.as_ptr() as *const c_void, gl::STATIC_DRAW);
    
            let mut vao = 0;
            gl::GenVertexArrays(1, &mut vao);
            debug_assert!(vao > 0);
    
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_vbo);
            gl::VertexAttribPointer(v_pos_location, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());
            gl::BindBuffer(gl::ARRAY_BUFFER, tex_coords_vbo);
            gl::VertexAttribPointer(v_tex_location, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());
            gl::EnableVertexAttribArray(v_pos_location);
            gl::EnableVertexAttribArray(v_tex_location);
    
            debug_assert!(self.context.validate_shader_program(shader_program));
    
            vao
        };
    
        let tex = self.context.send_to_gpu_texture(
                self.frame_buffer().as_buffer(), 
                GlWrappingMode::GlRepeat
            ).unwrap();
        // The time elapsed since the last call to glfwGetTime().
        let mut current_time = 0_f64;
        while !self.context.window_should_close() {
            let (width, height) = self.context.get_framebuffer_size();
            let new_time = self.context.get_time();
            let time_elapsed = new_time - current_time;
            current_time = new_time;
            
            // let (scale_x, scale_y) = window.get_content_scale();
            // gui_scale_mat = Matrix4x4::from_affine_nonuniform_scale(scale_x, scale_y, 1_f32);
    
            // Poll for and process events
            self.context.poll_events();
            for (_, event) in glfw::flush_messages(&self.context.events) {
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        self.context.window.set_should_close(true);
                    },
                    _ => {},
                }
            }
    
            println!("Updating scene");
            self.update(time_elapsed);
            println!("Rendering scene");
            let now = SystemTime::now();
            self.render();
            let elapsed = now.elapsed().unwrap();
            println!("Rendering time = {:?}", elapsed);
            self.frame_buffer_mut().flip_vertical();
            self.context.update_to_gpu_texture(tex, self.frame_buffer().as_buffer());
           
            unsafe {
                let m_trans_location = gl::GetUniformLocation(shader_program, gl_str("m_trans").as_ptr());
                debug_assert!(m_trans_location > -1);
                let m_gui_scale_location = gl::GetUniformLocation(shader_program, gl_str("m_gui_scale").as_ptr());
                debug_assert!(m_gui_scale_location > -1);
        
                gl::UniformMatrix4fv(m_trans_location, 1, gl::FALSE, trans_mat.as_ptr());
                gl::UniformMatrix4fv(m_gui_scale_location, 1, gl::FALSE, gui_scale_mat.as_ptr());
        
                gl::Viewport(0, 0, width as i32, height as i32);
                gl::ClearBufferfv(gl::COLOR, 0, &[0.2_f32, 0.2_f32, 0.2_f32, 1.0_f32] as *const GLfloat);
                gl::UseProgram(shader_program);
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, tex);
                gl::BindVertexArray(vao);
                gl::DrawArrays(gl::TRIANGLES, 0, 6);
            }
    
            // Swap front and back buffers
            self.context.swap_buffers();
        }
    }
}

