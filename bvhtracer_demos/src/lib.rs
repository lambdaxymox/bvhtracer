extern crate bvhtracer;
extern crate cglinalg;


#[allow(clippy::all)]
mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

mod app;
mod context;
mod state;


pub use crate::app::*;
pub use crate::context::*;
pub use crate::state::*;


