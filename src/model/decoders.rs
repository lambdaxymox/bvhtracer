use crate::mesh::*;
use crate::materials::*;
use super::model::*;
use crate::texture_buffer::*;
use std::error;
use std::fmt;
use std::fs::{
    File,
};
use std::io::{
    Read,
};
use std::path::{
    Path,
};


pub type ModelResult<T> = Result<T, ModelError>;


#[derive(Debug)]
pub enum ModelError {
    Material(),
    Mesh(),
}

impl fmt::Display for ModelError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl error::Error for ModelError {}


pub trait ModelDecoder<'a>: Sized {
    fn read_model(self) -> ModelResult<ModelInstance>;
}

pub struct SimpleModelDecoder<R1, R2> {
    mesh_reader: R1,
    material_reader: R2,
}

impl<'a, R1, R2> SimpleModelDecoder<R1, R2> 
where
    R1: Read + 'a,
    R2: Read + 'a,
{
    pub fn new(mesh_reader: R1, material_reader: R2) -> Self {
        Self { mesh_reader, material_reader, }
    }
}

impl<'a, R1, R2> ModelDecoder<'a> for SimpleModelDecoder<R1, R2> 
where
    R1: Read + 'a,
    R2: Read + 'a,
{
    fn read_model(self) -> ModelResult<ModelInstance> {
        let obj_decoder = ObjMeshDecoder::new(self.mesh_reader);
        let mesh = obj_decoder.read_mesh().unwrap();
        let texture_decoder: PngTextureBufferDecoder<Rgb<u8>, _> = PngTextureBufferDecoder::new(self.material_reader);
        let texture_buffer = texture_decoder.read_texture().unwrap();
        let texture = TextureMaterial::new(texture_buffer);
        let model = ModelBuilder::new()
            .with_mesh(mesh)
            .with_texture(texture)
            .build();

        Ok(model)
    }
}

