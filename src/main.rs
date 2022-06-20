extern crate cglinalg;
extern crate rand;
extern crate rand_isaac;

use rand::prelude::{
    Rng,
};
use cglinalg::{
    Matrix4x4,
    Vector3,
    Magnitude,
};

mod bvhtracer;
mod canvas;


use crate::bvhtracer::*;
use crate::canvas::*;
use std::io;
use std::io::Write;
use std::fs::{
    File,
};
use std::ops;


const SCREEN_WIDTH: usize = 1280;
const SCREEN_HEIGHT: usize = 720;


fn initialize_scene(triangle_count: usize) -> Scene {
    let mut objects = vec![Triangle::default(); 64];
    let mut rng = rand::prelude::thread_rng();
    for i in 0..triangle_count {
        let r0 = Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>());
        let r1 = Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>());
        let r2 = Vector3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>());
        let vertex0 = r0 * 9_f32 - Vector3::from_fill(5_f32);
        let vertex1 = vertex0 + r1;
        let vertex2 = vertex0 + r2;
        let centroid = Vector3::zero();
        objects[i] = Triangle::new(vertex0, vertex1, vertex2, centroid);
    }

    let builder = SceneBuilder::new();
    builder.with_objects(objects).build()
}

fn write_image_to_file(canvas: &Canvas, file: &mut File) -> io::Result<()> {
    write!(file, "P3\n{} {}\n255\n", canvas.width, canvas.height).unwrap();
    for pixel in canvas.data.iter() {
        writeln!(file, "{} {} {}", pixel.r, pixel.g, pixel.b).unwrap();
    }

    Ok(())
}

fn render_naive() -> Canvas {
    let triangle_count = 64;
    let scene = initialize_scene(triangle_count);
    let mut canvas = Canvas::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    // Set up camera.
    let camera_position = Vector3::new(0_f32, 0_f32, -18_f32);
    let p0 = Vector3::new(-1_f32, 1_f32, -15_f32);
    let p1 = Vector3::new(1_f32, 1_f32, -15_f32);
    let p2 = Vector3::new(-1_f32, -1f32, -15_f32);

    println!("rendering scene.");
    for row in 0..SCREEN_HEIGHT {
        for col in 0..SCREEN_WIDTH {
            let pixel_position = p0 + 
                (p1 - p0) * (col as f32 / SCREEN_WIDTH as f32) + 
                (p2 - p0) * (row as f32 / SCREEN_HEIGHT as f32);
            let ray_origin = camera_position;
            let ray_direction = (pixel_position - ray_origin).normalize();
            let ray_t = f32::MAX;
            let ray = Ray::new(ray_origin, ray_direction, ray_t);
            for object in scene.objects.iter() {
                if let Some(intersected_ray) = object.intersect(&ray) { 
                    if intersected_ray.t < f32::MAX {
                        canvas[row][col] = Rgba::new(255, 255, 255, 255);
                    }
                }
            }
        }
    }

    canvas
}

fn render() -> Canvas {
    let triangle_count = 64;
    let scene = initialize_scene(triangle_count);
    let mut canvas = Canvas::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    // Set up camera.
    let camera_position = Vector3::new(0_f32, 0_f32, -18_f32);
    let p0 = Vector3::new(-1_f32, 1_f32, -15_f32);
    let p1 = Vector3::new(1_f32, 1_f32, -15_f32);
    let p2 = Vector3::new(-1_f32, -1f32, -15_f32);

    println!("rendering scene.");
    for row in 0..SCREEN_HEIGHT {
        for col in 0..SCREEN_WIDTH {
            let pixel_position = p0 + 
                (p1 - p0) * (col as f32 / SCREEN_WIDTH as f32) + 
                (p2 - p0) * (row as f32 / SCREEN_HEIGHT as f32);
            let ray_origin = camera_position;
            let ray_direction = (pixel_position - ray_origin).normalize();
            let ray_t = f32::MAX;
            let ray = Ray::new(ray_origin, ray_direction, ray_t);
            if let Some(intersected_ray) = scene.intersect(&ray) {
                if intersected_ray.t < f32::MAX {
                    canvas[row][col] = Rgba::new(255, 255, 255, 255);
                }
            }
        }
    }

    canvas
}



#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct BvhNode {
    aabb_min: Vector3<f32>,
    aabb_max: Vector3<f32>,
    left_node: usize,
    first_primitive_idx: usize,
    primitive_count: usize,
}

impl BvhNode {
    #[inline]
    pub fn is_leaf(&self) -> bool {
        self.primitive_count > 0
    }
}

pub struct Bvh {
    nodes: Vec<BvhNode>,
    node_indices: Vec<usize>,
    root_node_idx: usize,
    nodes_used: usize,
}

fn intersect_aabb(ray: &Ray, aabb_min: &Vector3<f32>, aabb_max: &Vector3<f32>) -> bool {
    let t_x1 = (aabb_min.x - ray.origin.x) / ray.direction.x;
    let t_x2 = (aabb_max.x - ray.origin.x) / ray.direction.x;
    let t_min = f32::min(t_x1, t_x2);
    let t_max = f32::max(t_x1, t_x2);
    let t_y1 = (aabb_min.y - ray.origin.y) / ray.direction.y; 
    let t_y2 = (aabb_max.y - ray.origin.y) / ray.direction.y;
    let t_min = f32::max(t_min, f32::min(t_y1, t_y2)); 
    let t_max = f32::min(t_max, f32::max(t_y1, t_y2));
    let t_z1 = (aabb_min.z - ray.origin.z) / ray.direction.z;
    let t_z2 = (aabb_max.z - ray.origin.z) / ray.direction.z;
    let t_min = f32::max(t_min, f32::min(t_z1, t_z2)); 
    let t_max = f32::min(t_max, f32::max(t_z1, t_z2));
    
    (t_max >= t_min) && (t_min < ray.t) && (t_max > 0_f32)
}

impl Bvh {
    pub fn intersect(&self, objects: &[Triangle], ray: &Ray, node_idx: usize) -> Option<Ray> {
        let node = &self.nodes[node_idx];
        if !intersect_aabb(ray, &node.aabb_min, &node.aabb_max) {
            return None;
        }
        if node.is_leaf() {
            for i in 0..node.primitive_count {
                let primitive_idx = self.node_indices[node.first_primitive_idx + i];
                if let Some(new_ray) = objects[primitive_idx].intersect(ray) {
                    return Some(new_ray);
                }
            }

            None
        } else {
            if let Some(new_ray) = self.intersect(objects, ray, node.left_node) {
                Some(new_ray) 
            } else if let Some(new_ray) = self.intersect(objects, ray, node.left_node + 1) {
                Some(new_ray)
            } else {
                return None;
            }
        }
    }
}

pub struct BvhBuilder {
    partial_bvh: Bvh,
}

impl BvhBuilder {
    pub fn new() -> Self {
        let nodes = vec![];
        let node_indices = vec![];
        let root_node_idx = 0;
        let nodes_used = 1;

        let partial_bvh = Bvh { nodes, node_indices, root_node_idx, nodes_used, };

        Self { partial_bvh, }
    }

    fn update_node_bounds(&mut self, objects: &mut [Triangle], node_idx: usize) {
        #[inline]
        fn min(vector1: &Vector3<f32>, vector2: &Vector3<f32>) -> Vector3<f32> {
            Vector3::new(
                f32::min(vector1.x, vector2.x),
                f32::min(vector1.y, vector2.y),
                f32::min(vector1.z, vector2.z),
            )
        }

        #[inline]
        fn max(vector1: &Vector3<f32>, vector2: &Vector3<f32>) -> Vector3<f32> {
            Vector3::new(
                f32::max(vector1.x, vector2.x),
                f32::max(vector1.y, vector2.y),
                f32::max(vector1.z, vector2.z),
            )
        }

        let first = {
            let mut node = &mut self.partial_bvh.nodes[node_idx];
            node.aabb_min = Vector3::from_fill(f32::MAX); 
            node.aabb_max = Vector3::from_fill(-f32::MAX);
            let first = node.first_primitive_idx;

            first
        };

        for i in 0..self.partial_bvh.nodes[node_idx].primitive_count {
            let leaf_triangle_idx = self.partial_bvh.node_indices[first + i];
            let leaf_triangle = &objects[leaf_triangle_idx];
            let node = &mut self.partial_bvh.nodes[node_idx];
            node.aabb_min = min(&node.aabb_min, &leaf_triangle.vertex0);
            node.aabb_min = min(&node.aabb_min, &leaf_triangle.vertex1);
            node.aabb_min = min(&node.aabb_min, &leaf_triangle.vertex2);
            node.aabb_max = max(&node.aabb_max, &leaf_triangle.vertex0);
            node.aabb_max = max(&node.aabb_max, &leaf_triangle.vertex1);
            node.aabb_max = max(&node.aabb_max, &leaf_triangle.vertex2);
        }
    }

    fn subdivide(&mut self, objects: &mut [Triangle], node_idx: usize) {
        // Terminate recursion.
        let (left_count, i) = {
            let node = &mut self.partial_bvh.nodes[node_idx];
            if node.primitive_count <= 2 {
                return;
            }
            // Determine the split axis and position.
            let extent = node.aabb_max - node.aabb_min;
            let mut axis = 0;
            if extent.y > extent.x {
                axis = 1;
            } 
            if extent.z > extent[axis] {
                axis = 2;
            }
            let split_pos = node.aabb_min[axis] + extent[axis] * (1_f32 / 2_f32);
            // In-place partition.
            let mut i = node.first_primitive_idx;
            let mut j = i + node.primitive_count - 1;
            while i <= j {
                if objects[i].centroid[axis] < split_pos {
                    i += 1;
                } else {
                    objects.swap(i, j);
                    j -= 1;
                }
            }

            // Abort split if one of the sides is empty.
            let left_count = i - node.first_primitive_idx;
            if left_count == 0 || left_count == node.primitive_count {
                return;
            }

            (left_count, i)
        };
        // Create child nodes.
        let left_child_idx = {
            self.partial_bvh.nodes_used += 1;
            self.partial_bvh.nodes_used
        };
        let right_child_idx = {
            self.partial_bvh.nodes_used += 1;
            self.partial_bvh.nodes_used
        };
        {
            self.partial_bvh.nodes[left_child_idx].first_primitive_idx = self.partial_bvh.nodes[node_idx].first_primitive_idx;
            self.partial_bvh.nodes[left_child_idx].primitive_count = left_count;
            self.partial_bvh.nodes[right_child_idx].first_primitive_idx = i;
            self.partial_bvh.nodes[right_child_idx].primitive_count = self.partial_bvh.nodes[node_idx].primitive_count - left_count;
        }
        {
            let node = &mut self.partial_bvh.nodes[node_idx];
            node.left_node = left_child_idx;
            node.primitive_count = 0;
        }

        self.update_node_bounds(objects, left_child_idx);
        self.update_node_bounds(objects, right_child_idx);
        // Recurse
        self.subdivide(objects, left_child_idx);
        self.subdivide(objects, right_child_idx);
    }

    pub fn build_for(mut self, objects: &mut [Triangle]) -> Bvh {
        // Populate the triangle index array.
        for i in 0..objects.len() {
            // self.partial_bvh.node_indices[i] = i;
            self.partial_bvh.node_indices.push(i);
        }

        self.partial_bvh.nodes = vec![BvhNode::default(); 2 * objects.len()];
        
        for object in objects.iter_mut() {
            let one_third = 1_f32 / 3_f32;
            object.centroid = (object.vertex0 + object.vertex1 + object.vertex2) * one_third;
        }
        // let len_nodes = self.partial_bvh.nodes.len();
        let root_node_idx = self.partial_bvh.root_node_idx;
        let mut root_node: &mut BvhNode = &mut self.partial_bvh.nodes[root_node_idx];
        root_node.left_node = 0;
        root_node.primitive_count = objects.len();

        self.update_node_bounds(objects, self.partial_bvh.root_node_idx);
        self.subdivide(objects, self.partial_bvh.root_node_idx);

        self.partial_bvh
    }
}


pub struct SceneBuilder {
    objects: Vec<Triangle>,
    bvh_builder: BvhBuilder
}

impl SceneBuilder {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bvh_builder: BvhBuilder::new(),
        }
    }

    pub fn with_objects(mut self, objects: Vec<Triangle>) -> Self {
        self.objects = objects;

        self
    }

    pub fn build(mut self) -> Scene {
        let bvh = self.bvh_builder.build_for(&mut self.objects);

        Scene::new(self.objects, bvh)
    }
}


pub struct Scene {
    pub objects: Vec<Triangle>,
    pub bvh: Bvh,
}

impl Scene {
    pub fn new(objects: Vec<Triangle>, bvh: Bvh) -> Self {
        Self { objects, bvh, }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Ray> {
        self.bvh.intersect(&self.objects, ray, self.bvh.root_node_idx)
    }
}






extern crate glfw;

mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

use glfw::{Action, Context, Key};
use gl::types::{GLuint, GLfloat};
use std::ffi::{CStr, CString, c_void};
use std::ptr;
use std::mem;
use std::fmt;

// OpenGL extension constants.
const GL_TEXTURE_MAX_ANISOTROPY_EXT: u32 = 0x84FE;
const GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT: u32 = 0x84FF;

fn gl_str(st: &str) -> CString {
    CString::new(st).unwrap()
}

/// A record containing all the relevant compilation log information for a
/// given GLSL shader program compiled at run time.
pub struct ShaderProgramLog {
    index: GLuint,
    log: String,
}

impl fmt::Display for ShaderProgramLog {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        writeln!(formatter, "Program info log for GL index {}:", self.index).unwrap();
        writeln!(formatter, "{}", self.log)
    }
}

/// Query the shader program information log generated during shader compilation
/// from OpenGL.
pub fn program_info_log(index: GLuint) -> ShaderProgramLog {
    let mut actual_length = 0;
    unsafe {
        gl::GetProgramiv(index, gl::INFO_LOG_LENGTH, &mut actual_length);
    }
    let mut raw_log = vec![0 as i8; actual_length as usize];
    unsafe {
        gl::GetProgramInfoLog(index, raw_log.len() as i32, &mut actual_length, &mut raw_log[0]);
    }

    let mut log = String::new();
    for i in 0..actual_length as usize {
        log.push(raw_log[i] as u8 as char);
    }

    ShaderProgramLog { index: index, log: log }
}

/// Validate that the shader program `sp` can execute with the current OpenGL program state.
/// Use this for information purposes in application development. Return `true` if the program and
/// OpenGL state contain no errors.
pub fn validate_shader_program(sp: GLuint) -> bool {
    let mut params = -1;
    unsafe {
        gl::ValidateProgram(sp);
        gl::GetProgramiv(sp, gl::VALIDATE_STATUS, &mut params);
    }

    if params != gl::TRUE as i32 {
        println!("Program {} GL_VALIDATE_STATUS = GL_FALSE\n", sp);
        println!("{}", program_info_log(sp));
        
        return false;
    }

    println!("Program {} GL_VALIDATE_STATUS = {}\n", sp, params);
    
    true
}

/// Load texture image into the GPU.
fn send_to_gpu_texture(canvas: &Canvas, wrapping_mode: GLuint) -> Result<GLuint, String> {
    let mut tex = 0;
    unsafe {
        gl::GenTextures(1, &mut tex);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, tex);
        gl::TexImage2D(
            gl::TEXTURE_2D, 0, gl::RGBA as i32, canvas.width as i32, canvas.height as i32, 0,
            gl::RGBA, gl::UNSIGNED_BYTE,
            canvas.as_ptr() as *const c_void
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrapping_mode as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrapping_mode as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
    }
    debug_assert!(tex > 0);

    let mut max_aniso = 0.0;
    unsafe {
        gl::GetFloatv(GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT, &mut max_aniso);
        // Set the maximum!
        gl::TexParameterf(gl::TEXTURE_2D, GL_TEXTURE_MAX_ANISOTROPY_EXT, max_aniso);
    }

    Ok(tex)
}

fn main() -> io::Result<()> {
    use std::time;
    let now = time::SystemTime::now();
    let canvas = render_naive();
    let elapsed_naive = now.elapsed().unwrap();
    let now = time::SystemTime::now();
    let canvas = render();
    let elapsed_bvh = now.elapsed().unwrap();
    println!("elapsed_naive = {}", elapsed_naive.as_micros());
    println!("elapsed_bvh = {}", elapsed_bvh.as_micros());

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // We must place the window hints before creating the window because
    // glfw cannot change the properties of a window after it has been created.
    glfw.window_hint(glfw::WindowHint::Resizable(false));
    glfw.window_hint(glfw::WindowHint::Samples(Some(4)));
    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(
            SCREEN_WIDTH as u32, 
            SCREEN_HEIGHT as u32, 
            "GLFW Window", 
            glfw::WindowMode::Windowed
        )
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);
    window.set_size_polling(true);
    window.set_refresh_polling(true);
    window.set_size_polling(true);
    window.set_sticky_keys(true);

    // Load the OpenGl function pointers.
    gl::load_with(|symbol| { window.get_proc_address(symbol) as *const _ });

    let vertices: Vec<[f32; 2]> = vec![
        [1.0, 1.0], [-1.0, -1.0], [ 1.0, -1.0], 
        [1.0, 1.0], [-1.0,  1.0], [-1.0, -1.0],
    ];
    let tex_coords: Vec<[f32; 2]> = vec![
        [1.0, 1.0], [0.0, 0.0], [1.0, 0.0],
        [1.0, 1.0], [0.0, 1.0], [0.0, 0.0],
    ];

    let trans_mat = Matrix4x4::identity();
    let mut gui_scale_mat = Matrix4x4::identity();

    let shader_program = unsafe {
        let vertex_shader_source = include_bytes!("shaders.vert.glsl");
        let p = vertex_shader_source.as_ptr() as *const i8;
        let length = vertex_shader_source.len() as i32;
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vertex_shader, 1, &p, [length].as_ptr());
        gl::CompileShader(vertex_shader);

        let fragment_shader_source = include_bytes!("shaders.frag.glsl");
        let p = fragment_shader_source.as_ptr() as *const i8;
        let length = fragment_shader_source.len() as i32;
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(fragment_shader, 1, &p, [length].as_ptr());
        gl::CompileShader(fragment_shader);
       
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        shader_program
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
        gl::VertexAttribPointer(v_tex_location, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());
        gl::EnableVertexAttribArray(v_pos_location);
        gl::EnableVertexAttribArray(v_tex_location);

        debug_assert!(validate_shader_program(shader_program));

        vao
    };

    let tex = send_to_gpu_texture(&canvas, gl::REPEAT).unwrap();

    // Loop until the user closes the window
    while !window.should_close() {
        let (width, height) = window.get_framebuffer_size();
        // let time_elapsed = glfw.get_time();
        let (scale_x, scale_y) = window.get_content_scale();
        gui_scale_mat = Matrix4x4::from_affine_nonuniform_scale(scale_x, scale_y, 1_f32);

        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                },
                _ => {},
            }
        }
       
        unsafe {
            let m_trans_location = gl::GetUniformLocation(shader_program, gl_str("m_trans").as_ptr());
            debug_assert!(m_trans_location > -1);
            let m_gui_scale_location = gl::GetUniformLocation(shader_program, gl_str("m_gui_scale").as_ptr());
            debug_assert!(m_gui_scale_location > -1);
    
            gl::UniformMatrix4fv(m_trans_location, 1, gl::FALSE, trans_mat.as_ptr());
            gl::UniformMatrix4fv(m_gui_scale_location, 1, gl::FALSE, gui_scale_mat.as_ptr());
    
            gl::Viewport(0, 0, width, height);
            gl::ClearBufferfv(gl::COLOR, 0, &[0.2_f32, 0.2_f32, 0.2_f32, 1.0_f32] as *const GLfloat);
            gl::UseProgram(shader_program);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, tex);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }

        // Swap front and back buffers
        window.swap_buffers();
    }

    Ok(())
}

