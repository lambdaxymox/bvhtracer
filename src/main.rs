extern crate cglinalg;
extern crate rand;
extern crate rand_isaac;
extern crate stb_image;

use rand::prelude::IteratorRandom;
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
mod scene;


use crate::bvhtracer::*;
use crate::canvas::*;
use crate::scene::*;
use std::io;
use std::io::{
    BufReader,
    Read,
    Write,
};
use std::fs::{
    File,
};
use std::ops;
use std::num::{
    ParseFloatError,
};
use std::path::{
    Path,
};
use stb_image::image;


const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 640;


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
        objects[i] = Triangle::new(vertex0, vertex1, vertex2);
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

fn test_scene() -> Scene {
    let triangle = Triangle::new(
        Vector3::new(0_f32, 1_f32 / 2_f32, 0_f32),
        Vector3::new(-1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
        Vector3::new(1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
    );
    let triangles = vec![triangle];
    let builder = SceneBuilder::new();
    
    builder.with_objects(triangles).build()
}

fn render_test_scene(scene: &Scene) -> Canvas {
    // TODO: Put this stuff into an actual camera type, and place data into the scene construction.
    let mut canvas = Canvas::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    // Set up camera.
    let camera_position = Vector3::new(0_f32, 0_f32, 2_f32);
    let p0 = Vector3::new(-1_f32, 1_f32, 1_f32);
    let p1 = Vector3::new(1_f32, 1_f32, 1_f32);
    let p2 = Vector3::new(-1_f32, -1_f32, 1_f32);

    println!("rendering scene.");
    for row in 0..SCREEN_HEIGHT {
        for col in 0..SCREEN_WIDTH {
            let ray_origin = camera_position;
            let pixel_position = p0 + 
                (p1 - p0) * (col as f32 / SCREEN_WIDTH as f32) + 
                (p2 - p0) * (row as f32 / SCREEN_HEIGHT as f32);
            // let ray_origin = camera_position;
            let ray_direction = (pixel_position - ray_origin).normalize();
            let ray_t = f32::MAX;
            let ray = Ray::new(ray_origin, ray_direction, ray_t);
            if let Some(intersected_ray) = scene.intersect(&ray) {
                if intersected_ray.t < f32::MAX {
                    let color = 500 - ((intersected_ray.t * 42_f32) as i32);
                    let c = color * 0x010101;
                    let r = ((c & 0x00FF0000) >> 16) as u8;
                    let g = ((c & 0x0000FF00) >> 8) as u8;
                    let b = (c & 0x000000FF) as u8;
                    canvas[row][col] = Rgba::new(r, g, b, 255);
                }
            }
        }
    }

    canvas
}

fn test_scene2() -> Scene {
    let triangle1 = Triangle::new(
        Vector3::new(0_f32, 1_f32 / 2_f32, 0_f32),
        Vector3::new(-1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
        Vector3::new(1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
    );
    let triangle2 = Triangle::new(
        Vector3::new(2_f32 / f32::sqrt(3_f32), 1_f32, -0.2_f32),
        Vector3::new(-2_f32 / f32::sqrt(3_f32), 1_f32, -0.2_f32),
        Vector3::new(0_f32, -1_f32, -0.2_f32),
    );
    let triangles = vec![triangle1, triangle2];
    let builder = SceneBuilder::new();
    
    builder.with_objects(triangles).build()
}

fn render_test_scene2(scene: &Scene) -> Canvas {
    // TODO: Put this stuff into an actual camera type, and place data into the scene construction.
    let mut canvas = Canvas::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    // Set up camera.
    let camera_position = Vector3::new(0_f32, 0_f32, 2_f32);
    let p0 = Vector3::new(-1_f32, 1_f32, 1_f32);
    let p1 = Vector3::new(1_f32, 1_f32, 1_f32);
    let p2 = Vector3::new(-1_f32, -1_f32, 1_f32);

    println!("rendering scene.");
    for row in 0..SCREEN_HEIGHT {
        for col in 0..SCREEN_WIDTH {
            let ray_origin = camera_position;
            let pixel_position = p0 + 
                (p1 - p0) * (col as f32 / SCREEN_WIDTH as f32) + 
                (p2 - p0) * (row as f32 / SCREEN_HEIGHT as f32);
            // let ray_origin = camera_position;
            let ray_direction = (pixel_position - ray_origin).normalize();
            let ray_t = f32::MAX;
            let ray = Ray::new(ray_origin, ray_direction, ray_t);
            if let Some(intersected_ray) = scene.intersect(&ray) {
                if intersected_ray.t < f32::MAX {
                    let color = 500 - ((intersected_ray.t * 42_f32) as i32);
                    let c = color * 0x010101;
                    let r = ((c & 0x00FF0000) >> 16) as u8;
                    let g = ((c & 0x0000FF00) >> 8) as u8;
                    let b = (c & 0x000000FF) as u8;
                    canvas[row][col] = Rgba::new(r, g, b, 255);
                }
            }
        }
    }

    canvas
}

fn render_depth_unity(scene: &Scene) -> Canvas {
    // TODO: Put this stuff into an actual camera type, and place data into the scene construction.
    let mut canvas = Canvas::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    // Set up camera.
    let camera_position = Vector3::new(-1.5, -0.2, -2.5);
    let p0 = Vector3::new(-1_f32, 1_f32, 2_f32);
    let p1 = Vector3::new(1_f32, 1_f32, 2_f32);
    let p2 = Vector3::new(-1_f32, -1_f32, 2_f32);

    println!("rendering scene.");
    for row in 0..SCREEN_HEIGHT {
        for col in 0..SCREEN_WIDTH {
            let pixel_position = camera_position + p0 + 
                (p1 - p0) * (col as f32 / SCREEN_WIDTH as f32) + 
                (p2 - p0) * (row as f32 / SCREEN_HEIGHT as f32);
            let ray_origin = camera_position;
            let ray_direction = (pixel_position - ray_origin).normalize();
            let ray_t = f32::MAX;
            let ray = Ray::new(ray_origin, ray_direction, ray_t);
            if let Some(intersected_ray) = scene.intersect(&ray) {
                if intersected_ray.t < f32::MAX {
                    let color = 500 - ((intersected_ray.t * 42_f32) as i32);
                    let c = color * 0x010101;
                    let r = ((c & 0x00FF0000) >> 16) as u8;
                    let g = ((c & 0x0000FF00) >> 8) as u8;
                    let b = (c & 0x000000FF) as u8;
                    canvas[row][col] = Rgba::new(r, g, b, 255);
                }
            }
        }
    }

    canvas
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

fn load_tri_file_model<P: AsRef<Path>>(path: P) -> Result<Vec<Triangle>, (usize, ParseFloatError)> {
    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let mut triangles = vec![];
    for line in contents.split('\n') {
        let mut elements = [0_f32; 9];
        for (i, element_i) in line.split(' ').enumerate() {
            elements[i] = match element_i.parse::<f32>() {
                Ok(value) => value,
                Err(err) => return Err((i, err)),
            };
        }

        let mut vertices = [Vector3::zero(); 3];
        for (i, vertex_i) in elements
            .chunks(3)
            .map(|chunk| Vector3::new(chunk[0], chunk[1], chunk[2]))
            .enumerate() 
        {
            vertices[i] = vertex_i;
        }

        let triangle = Triangle::new(vertices[0], vertices[1], vertices[2]);

        triangles.push(triangle);
    }

    Ok(triangles)
}

fn main() -> io::Result<()> {

    let triangles = load_tri_file_model("assets/unity.tri").unwrap();
    let builder = SceneBuilder::new();
    let scene = builder.with_objects(triangles).build();
    let mut canvas = render_depth_unity(&scene);
    let mut file = File::create("output.ppm").unwrap();
    write_image_to_file(&canvas, &mut file);

    /*
    let scene = test_scene2();
    let mut canvas = render_test_scene2(&scene);
    let mut file = File::create("output.ppm").unwrap();
    write_image_to_file(&canvas, &mut file)?;
    */
    /*
    let mut canvas = Canvas::new(2, 2);
    canvas[0][0] = Rgba::new(255, 0, 0, 255);
    canvas[0][1] = Rgba::new(0, 255, 0, 255);
    canvas[1][0] = Rgba::new(0, 0, 255, 255);
    canvas[1][1] = Rgba::new(255, 255, 0, 255);
    let mut file = File::create("output.ppm").unwrap();
    write_image_to_file(&canvas, &mut file)?;
    */
    /*
    let mut canvas = Canvas::new(2, 2);
    canvas[1][1] = Rgba::new(255, 0, 0, 255);
    canvas[1][0] = Rgba::new(0, 255, 0, 255);
    canvas[0][1] = Rgba::new(0, 0, 255, 255);
    canvas[0][0] = Rgba::new(255, 255, 0, 255);
    let mut file = File::create("output.ppm").unwrap();
    write_image_to_file(&canvas, &mut file)?;
    */

    let height = canvas.height;
    // let width_in_bytes = 3 * canvas.width;
    let width = canvas.width;
    let half_height = canvas.height / 2;
    for row in 0..half_height {
        for col in 0..width {
            let temp = canvas.data[row * width + col];
            canvas.data[row * width + col] = canvas.data[((height - row - 1) * width) + col];
            canvas.data[((height - row - 1) * width) + col] = temp;
        }
    }


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
        gl::BindBuffer(gl::ARRAY_BUFFER, tex_coords_vbo);
        gl::VertexAttribPointer(v_tex_location, 2, gl::FLOAT, gl::FALSE, 0, ptr::null());
        gl::EnableVertexAttribArray(v_pos_location);
        gl::EnableVertexAttribArray(v_tex_location);

        debug_assert!(validate_shader_program(shader_program));

        vao
    };

    let tex = send_to_gpu_texture(&canvas, gl::REPEAT).unwrap();
    // let tex = send_to_gpu_texture_stb(&canvas, gl::CLAMP_TO_EDGE).unwrap();

    // Loop until the user closes the window
    while !window.should_close() {
        let (width, height) = window.get_framebuffer_size();
        // let time_elapsed = glfw.get_time();
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

