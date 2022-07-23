extern crate cglinalg;
extern crate wavefront_obj;
extern crate rand;
extern crate rand_isaac;
extern crate image;
extern crate tri_loader;
extern crate tiled_array;
extern crate num_traits;
extern crate glfw;


#[allow(clippy::all)]
mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

mod aabb;
mod backend;
mod triangle;
mod pixel;
mod frame_buffer;
mod ppm;
mod ray;
mod bvh;
mod mesh;
mod model;
mod scene;
mod tlas;
mod texture;
// mod camera;

use crate::backend::*;
use crate::triangle::*;
use crate::frame_buffer::*;
use crate::pixel::*;
use crate::model::*;
use crate::scene::*;

use cglinalg::{
    Magnitude,
    Matrix4x4,
    Vector3,
    Radians,
};
use glfw::{Action, Context, Key};
use gl::types::{
    GLuint, 
    GLfloat
};
use rand::{
    Rng,
    SeedableRng, 
};
use rand_isaac::{
    IsaacRng,
};
use std::io;
use std::mem;
use std::path::{
    Path,
};
use std::ptr;
use std::ffi::{
    c_void,
};


const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 640;


pub trait AppState {
    fn update(&mut self, elapsed: f64);

    fn active_scene(&self) -> &Scene;

    fn active_scene_mut(&mut self) -> &mut Scene;
}

struct App {
    frame_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    state: Box<dyn AppState>,
    renderer: Renderer,
}
    
impl App {
    fn new(state: Box<dyn AppState>, width: usize, height: usize) -> Self {
        let frame_buffer = ImageBuffer::from_fill(
            width, 
            height,
            Rgba::from([0, 0, 0, 1])
        );
        let renderer = Renderer::new();
    
        Self { frame_buffer, state, renderer, }
    }

    fn update(&mut self, elapsed: f64) {
        self.state.update(elapsed);
    }

    fn render(&mut self) -> usize {
        self.renderer.render(self.state.active_scene(), &mut self.frame_buffer)
    }
}

struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&mut self, scene: &Scene, frame_buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) -> usize {
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
                    let mut ray = scene.active_camera().get_ray_world_space(
                        (tile_width * x + u) as f32 / SCREEN_WIDTH as f32,
                        (tile_height * y + v) as f32 / SCREEN_HEIGHT as f32,
                    );
                    if let Some(t_intersect) = scene.intersect(&ray) {
                        ray.t = t_intersect;
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


struct AppStateBigBenClock {
    active_scene: Scene,
    r: f32,
    originals: Vec<Triangle<f32>>,
}

impl AppStateBigBenClock {
    fn new() -> Self {
        let focal_offset = Vector3::new(0_f32, 3.5_f32, 0_f32);
        let spec = IdentityModelSpec::new(
            -1_f32, 
            1_f32, 
            -1_f32, 
            1_f32, 
            2_f32, 
            -4.5_f32, 
            focal_offset
        );
        let camera = Camera::from_spec(spec);
        let mesh = load_tri_model("assets/bigben.tri");
        let model_builder = ModelBuilder::new();
        let model = model_builder.with_primitives(mesh).build();
        let object = SceneObjectBuilder::new(model)
            .build();
        let active_scene = SceneBuilder::new(camera)
            .with_object(object)
            .build();
        let r = 0_f32;
        let originals = active_scene.get_unchecked(0)
            .model()
            .mesh()
            .borrow()
            .primitives()
            .iter()
            .map(|p| *p)
            .collect::<Vec<_>>();

        Self { active_scene, r, originals, }
    }

    fn animate(&mut self) {
        self.r += 0.05;
        if self.r > std::f32::consts::FRAC_2_PI {
            self.r -= std::f32::consts::FRAC_2_PI;
        }
        let a = f32::sin(self.r) * 0.5;
        let mesh = self.active_scene().get_unchecked(0).model().mesh();
        for i in 0..self.originals.len() {
            let o_0 = self.originals[i].vertex0;
            let s_0 = a * (o_0.y - 0.2) * 0.2;
            let x_0 = o_0.x * f32::cos(s_0) - o_0.y * f32::sin(s_0);
            let y_0 = o_0.x * f32::sin(s_0) + o_0.y * f32::cos(s_0);

            let o_1 = self.originals[i].vertex1;
            let s_1 = a * (o_1.y - 0.2) * 0.2;
            let x_1 = o_1.x * f32::cos(s_1) - o_1.y * f32::sin(s_1);
            let y_1 = o_1.x * f32::sin(s_1) + o_1.y * f32::cos(s_1);

            let o_2 = self.originals[i].vertex2;
            let s_2 = a * (o_2.y - 0.2) * 0.2;
            let x_2 = o_2.x * f32::cos(s_2) - o_2.y * f32::sin(s_2);
            let y_2 = o_2.x * f32::sin(s_2) + o_2.y * f32::cos(s_2);

            mesh.borrow_mut().primitives_mut()[i] = Triangle::new(
                Vector3::new(x_0, y_0, o_0.z),
                Vector3::new(x_1, y_1, o_1.z),
                Vector3::new(x_2, y_2, o_2.z),
            );
        }
    }
}


impl AppState for AppStateBigBenClock {
    fn update(&mut self, _elapsed: f64) {
        self.animate();
        self.active_scene.get_mut_unchecked(0).model().refit();
    }

    fn active_scene(&self) -> &Scene {
        &self.active_scene
    }

    fn active_scene_mut(&mut self) -> &mut Scene {
        &mut self.active_scene
    }
}



struct AppStateTwoArmadillos {
    angle: f32,
    active_scene: Scene,
}

impl AppStateTwoArmadillos {
    fn new() -> Self {
        let focal_offset = Vector3::new(0_f32, 0.5_f32, 0_f32);
        let spec = IdentityModelSpec::new(
            -1_f32, 
            1_f32, 
            -1_f32, 
            1_f32, 
            2_f32, 
            -4.5_f32, 
            focal_offset
        );
        let camera = Camera::from_spec(spec);
        let mesh = load_tri_model("assets/armadillo.tri");
        let model_builder = ModelBuilder::new();
        let model = model_builder.with_primitives(mesh).build();
        let mut objects = vec![];
        let model1_transform = Matrix4x4::from_affine_translation(
            &Vector3::new(-1.3_f32, 0_f32, 0_f32)
        );
        let model2_transform = Matrix4x4::from_affine_translation(
            &Vector3::new(1.3_f32, 0_f32, 0_f32)
        );
        let model1 = model.clone();
        let model2 = model.clone();
        let object1 = SceneObjectBuilder::new(model1)
            .with_transform(&model1_transform)
            .build();
        let object2 = SceneObjectBuilder::new(model2)
            .with_transform(&model2_transform)
            .build();
        objects.push(object1);
        objects.push(object2);
        let active_scene = SceneBuilder::new(camera)
            .with_objects(objects)
            .build();
        let angle = 0_f32;

        Self { angle, active_scene }
    }
}

impl AppState for AppStateTwoArmadillos {
    fn update(&mut self, _elapsed: f64) {
        self.angle += 0.01;
        if self.angle > std::f32::consts::FRAC_2_PI {
            self.angle -= std::f32::consts::FRAC_2_PI;
        }
        self.active_scene.get_mut_unchecked(0).set_transform(
            &Matrix4x4::from_affine_translation(&Vector3::new(-1.3_f32, 0_f32, 0_f32))
        );
        self.active_scene.get_mut_unchecked(1).set_transform(&(
            Matrix4x4::from_affine_translation(&Vector3::new(1.3_f32, 0_f32, 0_f32)) *
            Matrix4x4::from_affine_angle_y(Radians(self.angle))
        ));

        self.active_scene.rebuild();
    }

    fn active_scene(&self) -> &Scene {
        &self.active_scene
    }

    fn active_scene_mut(&mut self) -> &mut Scene {
        &mut self.active_scene
    }
}


struct AppStateSixteenArmadillos {
    active_scene: Scene,
    a: Vec<f32>,
    h: Vec<f32>,
    s: Vec<f32>,
}

impl AppStateSixteenArmadillos {
    fn new() -> Self {
        let rot_mat = Matrix4x4::from_affine_angle_x(Radians(0.5));
        let _p0 = Vector3::new(-1_f32, 1_f32, 2_f32);
        let _p1 = Vector3::new(1_f32, 1_f32, 2_f32);
        let _p2 = Vector3::new(-1_f32, -1_f32, 2_f32);
        let p0 = (rot_mat * _p0.extend(1_f32)).contract();
        let p1 = (rot_mat * _p1.extend(1_f32)).contract();
        let p2 = (rot_mat * _p2.extend(1_f32)).contract();
        let focal_offset = Vector3::new(0_f32, 4.5_f32, 0_f32);
        let spec = IdentityModelSpec::new(
            p0.x, 
            p1.x, 
            p2.y,
            p0.y,
            p0.z, 
            -8.5_f32, 
            focal_offset
        );
        let camera = Camera::from_spec(spec);
        let mesh = load_tri_model("assets/armadillo.tri");
        let model_builder = ModelBuilder::new();
        let model = model_builder.with_primitives(mesh).build();
        let objects = (0..16).map(|_| {
                SceneObjectBuilder::new(model.clone())
                    .with_transform(&Matrix4x4::from_affine_scale(0.75))
                    .build()
            }).collect::<Vec<_>>();
        let active_scene = SceneBuilder::new(camera)
            .with_objects(objects)
            .build();
        let a = vec![0_f32; 16];
        let h = vec![
            5_f32, 4_f32, 3_f32, 2_f32, 1_f32, 5_f32, 4_f32, 3_f32, 
            5_f32, 4_f32, 3_f32, 2_f32, 1_f32, 5_f32, 4_f32, 3_f32,
        ];
        let s = vec![0_f32; 16];

        Self { active_scene, a, h, s }
    }
}

impl AppState for AppStateSixteenArmadillos {
    fn update(&mut self, _elapsed: f64) {
        let mut i = 0;
        for x in 0..4 {
            for y in 0..4 {
                let scale_mat = Matrix4x4::from_affine_scale(0.75);
                let trans_mat = Matrix4x4::from_affine_translation(
                    &Vector3::new((x as f32 - 1.5) * 2.5, 0_f32, (y as f32 - 1.5) * 2.5)
                );
                let rot_mat = if ((x + y) & 1) == 0 {
                    Matrix4x4::from_affine_angle_x(Radians(self.a[i])) * Matrix4x4::from_affine_angle_z(Radians(self.a[i]))
                } else {
                    Matrix4x4::from_affine_translation(&Vector3::new(0_f32, self.h[i / 2], 0_f32))
                };
                self.a[i] += (((i * 13) & 7 + 2) as f32) * 0.005;
                if self.a[i] > std::f32::consts::FRAC_2_PI {
                    self.a[i] -= std::f32::consts::FRAC_2_PI;
                }
                self.s[i] -= 0.01;
                self.h[i] += self.s[i];
                if (self.s[i] < 0_f32) && (self.h[i] < 0_f32) {
                    self.s[i] = 0.2;
                }

                let new_transform = trans_mat * rot_mat * scale_mat;
                self.active_scene.get_mut_unchecked(i).set_transform(&new_transform);

                i += 1;
            }
        } 

        self.active_scene.rebuild();
    }

    fn active_scene(&self) -> &Scene {
        &self.active_scene
    }

    fn active_scene_mut(&mut self) -> &mut Scene {
        &mut self.active_scene
    }
}


struct AppState256Armadillos {
    positions: Vec<Vector3<f32>>,
    directions: Vec<Vector3<f32>>,
    orientations: Vec<Vector3<f32>>,
    active_scene: Scene,
}

impl AppState256Armadillos {
    fn new() -> Self {
        let rot_mat = Matrix4x4::from_affine_angle_x(Radians(0.5));
        let _p0 = Vector3::new(-1_f32, 1_f32, 2_f32);
        let _p1 = Vector3::new(1_f32, 1_f32, 2_f32);
        let _p2 = Vector3::new(-1_f32, -1_f32, 2_f32);
        let p0 = (rot_mat * _p0.extend(1_f32)).contract();
        let p1 = (rot_mat * _p1.extend(1_f32)).contract();
        let p2 = (rot_mat * _p2.extend(1_f32)).contract();
        let focal_offset = Vector3::new(0_f32, 4.5_f32, 0_f32);
        let spec = IdentityModelSpec::new(
            p0.x, 
            p1.x, 
            p2.y,
            p0.y,
            p0.z, 
            -8.5_f32, 
            focal_offset
        );
        let camera = Camera::from_spec(spec);
        let mesh = load_tri_model("assets/armadillo.tri");
        let model_builder = ModelBuilder::new();
        let model = model_builder.with_primitives(mesh).build();
        let objects = (0..256).map(|_| {
                SceneObjectBuilder::new(model.clone())
                    .with_transform(&Matrix4x4::from_affine_scale(0.75))
                    .build()
            }).collect::<Vec<_>>();
        let active_scene = SceneBuilder::new(camera)
            .with_objects(objects)
            .build();

        let mut rng = IsaacRng::seed_from_u64(0);
        let mut positions = vec![Vector3::zero(); 256];
        let mut directions = vec![Vector3::zero(); 256];
        let mut orientations = vec![Vector3::zero(); 256];
        for i in 0..256 {
            positions[i] = Vector3::new(rng.gen(), rng.gen(), rng.gen());
            positions[i] -= Vector3::new(0.5, 0.5, 0.5);
            positions[i] *= 4_f32;
            directions[i] = positions[i].normalize() * 0.05;
            orientations[i] = Vector3::new(rng.gen(), rng.gen(), rng.gen());
            orientations[i] *= 2.5;
        }

        Self { positions, directions, orientations, active_scene }
    }
}

impl AppState for AppState256Armadillos {
    fn update(&mut self, _elapsed: f64) {
        for i in 0..256 {
            let rot_mat = Matrix4x4::from_affine_angle_x(Radians(self.orientations[i].x)) *
                Matrix4x4::from_affine_angle_y(Radians(self.orientations[i].y)) *
                Matrix4x4::from_affine_angle_z(Radians(self.orientations[i].z));
            let trans_mat = Matrix4x4::from_affine_translation(&self.positions[i]);
            let new_transform = trans_mat * rot_mat;
            self.active_scene.get_mut_unchecked(i).set_transform(&new_transform);
            self.positions[i] += self.directions[i];
            self.orientations[i] += self.directions[i];
            if self.positions[i].x < -3_f32 || self.positions[i].x > 3_f32 { 
                self.directions[i].x *= -1_f32;
            }
            if self.positions[i].y < -3_f32 || self.positions[i].y > 3_f32 { 
                self.directions[i].y *= -1_f32;
            }
            if self.positions[i].z < -3_f32 || self.positions[i].z > 3_f32 { 
                self.directions[i].z *= -1_f32;
            }
        }

        self.active_scene.rebuild();
    }

    fn active_scene(&self) -> &Scene {
        &self.active_scene
    }

    fn active_scene_mut(&mut self) -> &mut Scene {
        &mut self.active_scene
    }
}


/// Load texture image into the GPU.
fn send_to_gpu_texture(buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>, wrapping_mode: GLuint) -> Result<GLuint, String> {
    let mut tex = 0;
    unsafe {
        gl::GenTextures(1, &mut tex);
    }
    debug_assert!(tex > 0);
    unsafe {
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, tex);
        gl::TexImage2D(
            gl::TEXTURE_2D, 
            0, 
            gl::RGBA as i32, 
            buffer.width() as i32, 
            buffer.height() as i32, 
            0,
            gl::RGBA, 
            gl::UNSIGNED_BYTE,
            buffer.as_ptr() as *const c_void
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrapping_mode as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrapping_mode as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
    }

    let mut max_aniso = 0.0;
    unsafe {
        gl::GetFloatv(GL_MAX_TEXTURE_MAX_ANISOTROPY_EXT, &mut max_aniso);
    }
    let max_aniso_result = unsafe {
        gl::GetError()
    };
    debug_assert_eq!(max_aniso_result, gl::NO_ERROR);
    unsafe {
        // Set the maximum!
        gl::TexParameterf(gl::TEXTURE_2D, GL_TEXTURE_MAX_ANISOTROPY_EXT, max_aniso);
    }

    Ok(tex)
}

fn update_to_gpu_texture(tex: GLuint, buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>) {
    // SAFETY: This function does not check whether the texture handle actually exists on the GPU yet.
    // send_to_gpu_texture should be called first.
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, tex);
        gl::TexSubImage2D(
            gl::TEXTURE_2D, 
            0, 
            0, 
            0, 
            buffer.width() as i32, 
            buffer.height() as i32,
            gl::RGBA, 
            gl::UNSIGNED_BYTE, 
            buffer.as_ptr() as *const c_void
        );
        gl::BindTexture(gl::TEXTURE_2D, 0);
    }
    let err = unsafe {
        gl::GetError()
    };
    debug_assert_eq!(err, gl::NO_ERROR);
    unsafe {
        gl::GenerateMipmap(gl::TEXTURE_2D);
    }
}

fn load_tri_model<P: AsRef<Path>>(path: P) -> Vec<Triangle<f32>> {
    let loaded_tri_data = tri_loader::load(path).unwrap();
    loaded_tri_data.iter().map(|tri| {
        let vertex0 = Vector3::new(tri.vertex0.x, tri.vertex0.y, tri.vertex0.z);
        let vertex1 = Vector3::new(tri.vertex1.x, tri.vertex1.y, tri.vertex1.z);
        let vertex2 = Vector3::new(tri.vertex2.x, tri.vertex2.y, tri.vertex2.z);
        
        Triangle::new(vertex0, vertex1, vertex2)
    }).collect::<Vec<Triangle<_>>>()
}

fn main() -> io::Result<()> {
    use std::time::SystemTime;
    println!("Building scene.");
    let now = SystemTime::now();
    let state = Box::new(AppStateTwoArmadillos::new());
    let elapsed = now.elapsed().unwrap();
    println!("Scene building time = {:?}", elapsed);

    let mut app = App::new(state, SCREEN_WIDTH, SCREEN_HEIGHT);

    println!("Rendering scene.");
    let now = SystemTime::now();
    app.render();
    let elapsed = now.elapsed().unwrap();
    println!("Rendering time = {} s", elapsed.as_secs_f64());

    app.frame_buffer.flip_vertical();

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
        [1.0, 1.0], [-1.0,  1.0], [-1.0, -1.0],
        [1.0, 1.0], [-1.0, -1.0], [ 1.0, -1.0], 
    ];
    let tex_coords: Vec<[f32; 2]> = vec![
        [1.0, 1.0], [0.0, 1.0], [0.0, 0.0],
        [1.0, 1.0], [0.0, 0.0], [1.0, 0.0],
    ];

    let trans_mat = Matrix4x4::identity();
    let mut gui_scale_mat = Matrix4x4::identity();

    let shader_program = unsafe {
        let vertex_shader_source = include_bytes!("../shaders/shaders.vert.glsl");
        let p = vertex_shader_source.as_ptr() as *const i8;
        let length = vertex_shader_source.len() as i32;
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(vertex_shader, 1, &p, [length].as_ptr());
        gl::CompileShader(vertex_shader);

        let fragment_shader_source = include_bytes!("../shaders/shaders.frag.glsl");
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
        gl::BindBuffer(gl::ARRAY_BUFFER, tex_coords_vbo);
        gl::VertexAttribPointer(v_tex_location, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());
        gl::EnableVertexAttribArray(v_pos_location);
        gl::EnableVertexAttribArray(v_tex_location);

        debug_assert!(validate_shader_program(shader_program));

        vao
    };

    let tex = send_to_gpu_texture(&app.frame_buffer, gl::REPEAT).unwrap();
    // Loop until the user closes the window
    while !window.should_close() {
        let (width, height) = window.get_framebuffer_size();
        let time_elapsed = glfw.get_time();
        
        // let (scale_x, scale_y) = window.get_content_scale();
        // gui_scale_mat = Matrix4x4::from_affine_nonuniform_scale(scale_x, scale_y, 1_f32);

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

        println!("Updating scene");
        app.update(time_elapsed);
        println!("Rendering scene");
        let now = SystemTime::now();
        app.render();
        let elapsed = now.elapsed().unwrap();
        println!("Rendering time = {:?}", elapsed);
        app.frame_buffer.flip_vertical();
        update_to_gpu_texture(tex, &app.frame_buffer);
       
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

