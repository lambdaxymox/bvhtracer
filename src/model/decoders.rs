use crate::mesh::*;
use crate::materials::*;
use super::model::*;
use crate::texture_buffer::*;
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
    let obj_file = File::open(path).unwrap();
    let obj_decoder = ObjMeshDecoder::new(obj_file);
    
    obj_decoder.read_mesh().unwrap()
}

fn load_texture_from_png<P: AsRef<Path>>(path: P) -> TextureBuffer2D<Rgb<u8>, Vec<u8>> {
    let texture_file = File::open(&path).unwrap();
    let texture_decoder: PngTextureBufferDecoder<Rgb<u8>, _> = PngTextureBufferDecoder::new(texture_file);

    texture_decoder.read_texture().unwrap()
}

pub fn load_obj_model<P: AsRef<Path>>(obj_path: P, texture_path: P) -> ModelInstance {
    let mesh = load_mesh_from_obj(obj_path);
    let buffer = load_texture_from_png(texture_path);
    let texture = TextureMaterial::new(buffer);
    let builder = ModelBuilder::new()
        .with_mesh(mesh)
        .with_texture(texture);
    
    builder.build()
}

