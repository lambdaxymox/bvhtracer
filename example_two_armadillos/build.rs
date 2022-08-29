extern crate gl_generator;

use gl_generator::{Registry, Api, Profile, Fallbacks, GlobalGenerator};
use std::env;
use std::fs::File;
use std::path::Path;


#[cfg(target_os = "macos")]
fn register_gl_api(file: &mut File) {
    Registry::new(Api::Gl, (3, 3), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, file)
        .unwrap();
}

#[cfg(target_os = "windows")]
fn register_gl_api(file: &mut File) {
    Registry::new(Api::Gl, (4, 6), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, file)
        .unwrap();
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn register_gl_api(file: &mut File) {
    Registry::new(Api::Gl, (4, 6), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, file)
        .unwrap();
}

fn main() {
    let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join("gl_bindings.rs")).unwrap();

    register_gl_api(&mut file);
}

