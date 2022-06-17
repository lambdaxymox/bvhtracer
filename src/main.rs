extern crate cglinalg;
extern crate rand;
extern crate rand_isaac;

use rand::prelude::{
    Rng,
};
use cglinalg::{
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


const SCREEN_WIDTH: usize = 1280;
const SCREEN_HEIGHT: usize = 720;


fn intersect(triangle: &Triangle, ray: &Ray) -> Ray {
    let edge1 = triangle.vertex1 - triangle.vertex0;
    let edge2 = triangle.vertex2 - triangle.vertex0;
    let h = ray.direction.cross(&edge2);
    let a = edge1.dot(&h);
    if a > -0.0001 && a < 0.0001 {
        // The ray is parallel to the triangle.
        return *ray;
    }
    let f = 1_f32 / a;
    let s = ray.origin - triangle.vertex0;
    let u = f * s.dot(&h);
    if u < 0_f32 || u > 1_f32 {
        return *ray;
    }
    let q = s.cross(&edge1);
    let v = f * ray.direction.dot(&q);
    if v < 0_f32 || u + v > 1_f32 {
        return *ray;
    }
    let t = f * edge2.dot(&q);
    if t > 0.0001 {
        let t_intersect = f32::min(ray.t, t);

        return Ray::new(ray.origin, ray.direction, t_intersect);
    } else {
        return *ray;
    }
}

pub struct Scene {
    objects: Vec<Triangle>,
}

impl Scene {
    pub fn new(objects: Vec<Triangle>) -> Self {
        Self { objects, }
    }
}

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

    Scene::new(objects)
}

fn write_image_to_file(canvas: &Canvas, file: &mut File) -> io::Result<()> {
    write!(file, "P3\n{} {}\n255\n", canvas.width, canvas.height).unwrap();
    for pixel in canvas.data.iter() {
        writeln!(file, "{} {} {}", pixel.r, pixel.g, pixel.b).unwrap();
    }

    Ok(())
}

fn render() -> io::Result<()> {
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
                let intersected_ray = intersect(object, &ray);
                if intersected_ray.t < f32::MAX {
                    canvas[row][col] = Rgba::new(255, 255, 255, 255);
                }
            }
        }
    }

    println!("writing image to file.");
    let mut file = File::create("output.ppm").unwrap();
    write_image_to_file(&canvas, &mut file)
}
/*
fn main() -> io::Result<()> {
    render()
}
*/
extern crate glfw;

mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

use glfw::{Action, Context, Key};
use gl::types::{GLfloat};

fn main() {
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

    // Loop until the user closes the window
    while !window.should_close() {
        let (width, height) = window.get_framebuffer_size();
        let time_elapsed = glfw.get_time();
        println!("{}", time_elapsed);

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
            gl::Viewport(0, 0, width, height);
            gl::ClearBufferfv(gl::COLOR, 0, &[0.2_f32, 0.2_f32, 0.2_f32, 1.0_f32] as *const GLfloat);
        }

        // Swap front and back buffers
        window.swap_buffers();
    }
}