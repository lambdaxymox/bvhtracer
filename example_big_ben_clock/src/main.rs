extern crate bvhtracer;
extern crate bvhtracer_demos;
extern crate cglinalg;


const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 640;


mod state;

use bvhtracer::*;
use bvhtracer_demos::*;
use crate::state::*;
use cglinalg::{
    Vector3,
};
use std::io;


fn main() -> io::Result<()> {
    use std::time::SystemTime;
    println!("Building scene.");
    let now = SystemTime::now();
    let state = Box::new(AppStateBigBenClock::new());
    let elapsed = now.elapsed().unwrap();
    println!("Scene building time = {:?}", elapsed);

    let accumulator = Box::new(IntersectionAccumulator::new(
        Vector3::from_fill(1_f32), 
        Vector3::zero()
    ));
    let pixel_shader = Box::new(IntersectionShader::new(
        Rgba::new(255, 255, 255, 255), 
        Rgba::new(0, 0, 0, 255),
    ));
    let renderer = Renderer::new(Box::new(PathTracer::new()));
    let context = init_gl("OpenGL Window", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32).unwrap();
    let mut app = App::new(context, pixel_shader, accumulator, state, renderer, SCREEN_WIDTH, SCREEN_HEIGHT);
    app.run();

    Ok(())
}

