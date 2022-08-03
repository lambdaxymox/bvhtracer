use crate::mesh::*;
use crate::materials::*;
use super::model::*;

use wavefront_obj::obj;

use std::fs::{
    File,
};
use std::io::{
    Read,
};
use std::path::{
    Path,
};


fn load_mesh_from_obj<P: AsRef<Path>>(path: P) -> Mesh<f32> {
    let mut obj_file = File::open(path).unwrap();
    let mut buffer = String::new();
    obj_file.read_to_string(&mut buffer).unwrap();
    let obj_set = obj::parse(&buffer).unwrap();
    let object = &obj_set.objects[0];

    load_mesh(object)
}

fn load_texture_from_png<P: AsRef<Path>>(path: P) -> TextureImage2D {
    let mut texture_file = File::open(path).unwrap();
    
    from_png_reader(&mut texture_file)
}

pub fn load_obj_model<P: AsRef<Path>>(obj_path: P, texture_path: P) -> ModelInstance {
    let mesh = load_mesh_from_obj(obj_path);
    let texture = load_texture_from_png(texture_path);
    let builder = ModelBuilder::new()
        .with_mesh(mesh)
        .with_texture(texture);
    
    builder.build()
}

