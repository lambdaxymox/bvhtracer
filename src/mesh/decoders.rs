use crate::mesh::*;
use crate::model::*;
use crate::materials::*;
use crate::geometry::*;

use cglinalg::{
    Magnitude,
    Vector2,
    Vector3,
};
use image::{
    ImageDecoder,
};
use image::codecs::{
    png::PngDecoder,
    jpeg::JpegDecoder,
};
use wavefront_obj::obj;

use std::fs::{
    File,
};
use std::io;
use std::io::{
    Read,
};
use std::path::{
    Path,
};


pub fn load_tri_model<P: AsRef<Path>>(path: P) -> Mesh {
    let loaded_tri_data = tri_loader::load(path).unwrap();
    let primitives = loaded_tri_data.iter().map(|tri| {
            let vertex0 = Vector3::new(tri.vertex0.x, tri.vertex0.y, tri.vertex0.z);
            let vertex1 = Vector3::new(tri.vertex1.x, tri.vertex1.y, tri.vertex1.z);
            let vertex2 = Vector3::new(tri.vertex2.x, tri.vertex2.y, tri.vertex2.z);
        
            Triangle::new(vertex0, vertex1, vertex2)
        }).collect::<Vec<Triangle<_>>>();
    let tex_coords = vec![TriangleTexCoords::default(); primitives.len()];
    let normals = primitives.iter().map(|primitive| {
            let v0v2 = (primitive.vertex2 - primitive.vertex0).normalize();
            let v0v1 = (primitive.vertex1 - primitive.vertex0).normalize();
            let normal = v0v2.cross(&v0v1).normalize();

            TriangleNormals::new(normal, normal, normal)
        })
        .collect::<Vec<_>>();

    Mesh::new(primitives, tex_coords, normals)
}

// TODO: Calculate normals from vertex data in the case that they're missing?
pub fn load_mesh(object: &obj::Object) -> Mesh {
    let mut primitives = vec![];
    let mut tex_coords = vec![];
    let mut normals = vec![];
    for element in object.element_set.iter() {
        match element {
            obj::Element::Face(vtn1, vtn2, vtn3) => {
                let triples = [
                    object.get_vtn_triple(*vtn1).unwrap(),
                    object.get_vtn_triple(*vtn2).unwrap(),
                    object.get_vtn_triple(*vtn3).unwrap(),
                ];

                let mut primitive = Triangle::default();
                let mut tex_coord = TriangleTexCoords::default();
                let mut normal = TriangleNormals::default();
             
                match triples[0] {
                    obj::VTNTriple::V(vp) => {
                        primitive.vertex0 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                        tex_coord.uv0 = Vector2::zero();
                        normal.normal0 = Vector3::zero();
                    }
                    obj::VTNTriple::VT(vp, vt) => {
                        primitive.vertex0 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                        tex_coord.uv0 = Vector2::new(vt.u as f32, vt.v as f32);
                        normal.normal0 = Vector3::zero();
                    }
                    obj::VTNTriple::VN(vp, vn) => {
                        primitive.vertex0 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                        tex_coord.uv0 = Vector2::zero();
                        normal.normal0 = Vector3::new(vn.x as f32, vn.y as f32, vn.z as f32);
                    }
                    obj::VTNTriple::VTN(vp, vt, vn) => {
                        primitive.vertex0 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                        tex_coord.uv0 = Vector2::new(vt.u as f32, vt.v as f32);
                        normal.normal0 = Vector3::new(vn.x as f32, vn.y as f32, vn.z as f32);
                    }
                }
                match triples[1] {
                    obj::VTNTriple::V(vp) => {
                        primitive.vertex1 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                        tex_coord.uv1 = Vector2::zero();
                        normal.normal1 = Vector3::zero();
                    }
                    obj::VTNTriple::VT(vp, vt) => {
                        primitive.vertex1 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                        tex_coord.uv1 = Vector2::new(vt.u as f32, vt.v as f32);
                        normal.normal1 = Vector3::zero();
                    }
                    obj::VTNTriple::VN(vp, vn) => {
                        primitive.vertex1 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                        tex_coord.uv1 = Vector2::zero();
                        normal.normal1 = Vector3::new(vn.x as f32, vn.y as f32, vn.z as f32);
                    }
                    obj::VTNTriple::VTN(vp, vt, vn) => {
                        primitive.vertex1 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                        tex_coord.uv1 = Vector2::new(vt.u as f32, vt.v as f32);
                        normal.normal1 = Vector3::new(vn.x as f32, vn.y as f32, vn.z as f32);
                    }
                }
                match triples[2] {
                    obj::VTNTriple::V(vp) => {
                        primitive.vertex2 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                        tex_coord.uv2 = Vector2::zero();
                        normal.normal2 = Vector3::zero();
                    }
                    obj::VTNTriple::VT(vp, vt) => {
                        primitive.vertex2 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                        tex_coord.uv2 = Vector2::new(vt.u as f32, vt.v as f32);
                        normal.normal2 = Vector3::zero();
                    }
                    obj::VTNTriple::VN(vp, vn) => {
                        primitive.vertex2 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                        tex_coord.uv2 = Vector2::zero();
                        normal.normal2 = Vector3::new(vn.x as f32, vn.y as f32, vn.z as f32);
                    }
                    obj::VTNTriple::VTN(vp, vt, vn) => {
                        primitive.vertex2 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                        tex_coord.uv2 = Vector2::new(vt.u as f32, vt.v as f32);
                        normal.normal2 = Vector3::new(vn.x as f32, vn.y as f32, vn.z as f32);
                    }
                }

                primitives.push(primitive);
                tex_coords.push(tex_coord);
                normals.push(normal);
            }
            _ => {}
        }
    }

    Mesh::new(primitives, tex_coords, normals)
}

fn load_mesh_from_obj<P: AsRef<Path>>(path: P) -> Mesh {
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

