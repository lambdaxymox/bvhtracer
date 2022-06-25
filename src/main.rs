extern crate cglinalg;
extern crate rand;
extern crate rand_isaac;
extern crate stb_image;
extern crate tri_loader;
extern crate num_traits;


use rand::prelude::{
    Rng,
};
use cglinalg::{
    Matrix4x4,
    Vector3,
    Magnitude,
};

mod backend;
mod triangle;
mod image;
mod scene;

use crate::triangle::*;
use crate::image::*;
use crate::scene::*;
use std::io;
use std::io::{
    Write,
};
use std::fs::{
    File,
};
use std::path::{
    Path,
};
use backend::*;


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

fn write_image_to_file(buffer: &ImageBuffer<Rgba<u8>>, file: &mut File) -> io::Result<()> {
    write!(file, "P3\n{} {}\n255\n", buffer.width(), buffer.height()).unwrap();
    for pixel in buffer.data.iter() {
        writeln!(file, "{} {} {}", pixel.r(), pixel.g(), pixel.b()).unwrap();
    }

    Ok(())
}

fn render() -> ImageBuffer<Rgba<u8>> {
    let triangle_count = 64;
    let scene = initialize_scene(triangle_count);
    let mut buffer = ImageBuffer::from_fn(SCREEN_WIDTH, SCREEN_HEIGHT, |_, _| {
        Rgba::from([0, 0, 0, 1])
    });

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
                    buffer[row][col] = Rgba::new(255, 255, 255, 255);
                }
            }
        }
    }

    buffer
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

fn render_test_scene(scene: &Scene) -> ImageBuffer<Rgba<u8>> {
    // TODO: Put this stuff into an actual camera type, and place data into the scene construction.
    let mut buffer = ImageBuffer::from_fn(SCREEN_WIDTH, SCREEN_HEIGHT, |_, _| {
        Rgba::from([0, 0, 0, 1])
    });

    // Set up camera.
    let camera_position = Vector3::new(0_f32, 0_f32, 2_f32);
    let p0 = Vector3::new(-1_f32, 1_f32, 1_f32);
    let p1 = Vector3::new(1_f32, 1_f32, 1_f32);
    let p2 = Vector3::new(-1_f32, -1_f32, 1_f32);

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
                    let color = 500 - ((intersected_ray.t * 42_f32) as i32);
                    let c = color * 0x010101;
                    let r = ((c & 0x00FF0000) >> 16) as u8;
                    let g = ((c & 0x0000FF00) >> 8) as u8;
                    let b = (c & 0x000000FF) as u8;
                    buffer[row][col] = Rgba::new(r, g, b, 255);
                }
            }
        }
    }

    buffer
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

fn render_test_scene2(scene: &Scene) -> ImageBuffer<Rgba<u8>> {
    // TODO: Put this stuff into an actual camera type, and place data into the scene construction.
    let mut buffer = ImageBuffer::from_fn(SCREEN_WIDTH, SCREEN_HEIGHT, |_, _| {
        Rgba::from([0, 0, 0, 1])
    });

    // Set up camera.
    let camera_position = Vector3::new(0_f32, 0_f32, 2_f32);
    let p0 = Vector3::new(-1_f32, 1_f32, 1_f32);
    let p1 = Vector3::new(1_f32, 1_f32, 1_f32);
    let p2 = Vector3::new(-1_f32, -1_f32, 1_f32);

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
                    let color = 500 - ((intersected_ray.t * 42_f32) as i32);
                    let c = color * 0x010101;
                    let r = ((c & 0x00FF0000) >> 16) as u8;
                    let g = ((c & 0x0000FF00) >> 8) as u8;
                    let b = (c & 0x000000FF) as u8;
                    buffer[row][col] = Rgba::new(r, g, b, 255);
                }
            }
        }
    }

    buffer
}

fn render_depth_unity(scene: &Scene) -> ImageBuffer<Rgba<u8>> {
    // TODO: Put this stuff into an actual camera type, and place data into the scene construction.
    let mut buffer = ImageBuffer::from_fn(SCREEN_WIDTH, SCREEN_HEIGHT, |_, _| {
        Rgba::from([0, 0, 0, 1])
    });

    // Set up camera.
    let camera_position = Vector3::new(-1.5, 0.0, -2.5);
    let p0 = Vector3::new(-1_f32, 1_f32, 2_f32);
    let p1 = Vector3::new(1_f32, 1_f32, 2_f32);
    let p2 = Vector3::new(-1_f32, -1_f32, 2_f32);

    println!("rendering scene.");
    for row in (0..SCREEN_HEIGHT).step_by(4) {
        for col in (0..SCREEN_WIDTH).step_by(4) {
            for v in 0..4 {
                for u in 0..4 {
                    let pixel_position = camera_position + p0 + 
                        (p1 - p0) * ((col + u) as f32 / SCREEN_WIDTH as f32) + 
                        (p2 - p0) * ((row + v) as f32 / SCREEN_HEIGHT as f32);
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
                            buffer[row + v][col + u] = Rgba::new(r, g, b, 255);
                        }
                    }
                }
            }
        }
    }

    buffer
}


extern crate glfw;

mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

use glfw::{Action, Context, Key};
use gl::types::{
    GLuint, 
    GLfloat
};
use std::ffi::{
    c_void,
};
use std::ptr;
use std::mem;

/// Load texture image into the GPU.
fn send_to_gpu_texture(buffer: &ImageBuffer<Rgba<u8>>, wrapping_mode: GLuint) -> Result<GLuint, String> {
    let mut tex = 0;
    unsafe {
        gl::GenTextures(1, &mut tex);
    }
    debug_assert!(tex > 0);
    unsafe {
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, tex);
        gl::TexImage2D(
            gl::TEXTURE_2D, 0, gl::RGBA as i32, buffer.width() as i32, buffer.height() as i32, 0,
            gl::RGBA, gl::UNSIGNED_BYTE,
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

fn load_tri_model<P: AsRef<Path>>(path: P) -> Vec<Triangle> {
    let loaded_tri_data = tri_loader::load(path).unwrap();
    loaded_tri_data.iter().map(|tri| {
        let vertex0 = Vector3::new(tri.vertex0.x, tri.vertex0.y, tri.vertex0.z);
        let vertex1 = Vector3::new(tri.vertex1.x, tri.vertex1.y, tri.vertex1.z);
        let vertex2 = Vector3::new(tri.vertex2.x, tri.vertex2.y, tri.vertex2.z);
        
        Triangle::new(vertex0, vertex1, vertex2)
    }).collect::<Vec<Triangle>>()
}

fn main() -> io::Result<()> {
    let triangles = load_tri_model("assets/unity.tri");
    let builder = SceneBuilder::new();
    let scene = builder.with_objects(triangles).build();
    let mut buffer = render_depth_unity(&scene);
    let mut file = File::create("output.ppm").unwrap();
    write_image_to_file(&buffer, &mut file);

    let height = buffer.height();
    let width = buffer.width();
    let half_height = buffer.height() / 2;
    for row in 0..half_height {
        for col in 0..width {
            let temp = buffer.data[row * width + col];
            buffer.data[row * width + col] = buffer.data[((height - row - 1) * width) + col];
            buffer.data[((height - row - 1) * width) + col] = temp;
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

    let tex = send_to_gpu_texture(&buffer, gl::REPEAT).unwrap();

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

