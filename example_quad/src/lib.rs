extern crate bvhtracer;
extern crate bvhtracer_demos;
extern crate cglinalg;


#[allow(clippy::all)]
mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 640;

mod app;
mod context;
mod state;

use bvhtracer::*;
use bvhtracer_demos::*;
/*
use crate::app::*;
use crate::context::*;
*/
use crate::state::*;
use std::io;


pub fn run() -> io::Result<()> {
    use std::time::SystemTime;
    println!("Building scene.");
    let now = SystemTime::now();
    let state = Box::new(AppStateQuad::new());
    let elapsed = now.elapsed().unwrap();
    println!("Scene building time = {:?}", elapsed);    
    let accumulator = Box::new(TextureMaterialAccumulator::new());
    let pixel_shader = Box::new(RadianceToRgbShader::new());
    let renderer = Renderer::new(Box::new(PathTracer::new()));

    let context = init_gl("OpenGL Window", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32).unwrap();
    let mut app = App::new(context, pixel_shader, accumulator, state, renderer, SCREEN_WIDTH, SCREEN_HEIGHT);
    app.run();

    Ok(())
}

