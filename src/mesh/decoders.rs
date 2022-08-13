use crate::mesh::*;
use crate::geometry::*;

use cglinalg::{
    Magnitude,
    Vector2,
    Vector3,
};
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


pub type MeshResult<T> = Result<T, MeshError>;

#[derive(Clone, Debug)]
pub enum MeshError {

}

pub trait MeshDecoder<'a>: Sized {
    type Reader: Read + 'a;

    fn read_mesh(self) -> MeshResult<Mesh<f32>>;
}

pub struct TriMeshDecoder<R> {
    reader: R,
}

impl<R> TriMeshDecoder<R> 
where
    R: Read,
{
    pub fn new(reader: R) -> Self {
        Self { reader, }
    }
}

impl<'a, R> MeshDecoder<'a> for TriMeshDecoder<R> 
where
    R: Read + 'a,
{
    type Reader = R;

    fn read_mesh(mut self) -> MeshResult<Mesh<f32>> {
        let loaded_tri_data = tri_loader::from_reader(&mut self.reader).unwrap();
        let primitives = loaded_tri_data.iter().map(|tri| {
                let vertex0 = Vector3::new(tri.vertex0.x, tri.vertex0.y, tri.vertex0.z);
                let vertex1 = Vector3::new(tri.vertex1.x, tri.vertex1.y, tri.vertex1.z);
                let vertex2 = Vector3::new(tri.vertex2.x, tri.vertex2.y, tri.vertex2.z);
            
                Triangle::new(vertex0, vertex1, vertex2)
            }).collect::<Vec<Triangle<_>>>();
        let tex_coords = vec![TextureCoordinates::default(); primitives.len()];
        let normals = primitives.iter().map(|primitive| {
                let v0v2 = (primitive.vertex2 - primitive.vertex0).normalize();
                let v0v1 = (primitive.vertex1 - primitive.vertex0).normalize();
                let normal = v0v2.cross(&v0v1).normalize();
    
                Normals::from([normal, normal, normal])
            })
            .collect::<Vec<_>>();
    
        let mut builder = MeshBuilder::new();
        for i in 0..primitives.len() {
            builder = builder.with_primitive(
                primitives[i], 
                tex_coords[i], 
                normals[i]
            );
        }
    
        let mesh = builder.build();

        Ok(mesh)
    }
}


/*
pub fn load_tri_model<P: AsRef<Path>>(path: P) -> Mesh<f32> {
    let loaded_tri_data = tri_loader::load(path).unwrap();
    let primitives = loaded_tri_data.iter().map(|tri| {
            let vertex0 = Vector3::new(tri.vertex0.x, tri.vertex0.y, tri.vertex0.z);
            let vertex1 = Vector3::new(tri.vertex1.x, tri.vertex1.y, tri.vertex1.z);
            let vertex2 = Vector3::new(tri.vertex2.x, tri.vertex2.y, tri.vertex2.z);
        
            Triangle::new(vertex0, vertex1, vertex2)
        }).collect::<Vec<Triangle<_>>>();
    let tex_coords = vec![TextureCoordinates::default(); primitives.len()];
    let normals = primitives.iter().map(|primitive| {
            let v0v2 = (primitive.vertex2 - primitive.vertex0).normalize();
            let v0v1 = (primitive.vertex1 - primitive.vertex0).normalize();
            let normal = v0v2.cross(&v0v1).normalize();

            Normals::from([normal, normal, normal])
        })
        .collect::<Vec<_>>();

    let mut builder = MeshBuilder::new();
    for i in 0..primitives.len() {
        builder = builder.with_primitive(
            primitives[i], 
            tex_coords[i], 
            normals[i]
        );
    }

    builder.build()
}
*/

pub struct ObjMeshDecoder<R> {
    reader: R,
}

impl<R> ObjMeshDecoder<R> 
where
    R: Read,
{
    pub fn new(reader: R) -> Self {
        Self { reader, }
    }
}

impl<'a, R> MeshDecoder<'a> for ObjMeshDecoder<R> 
where
    R: Read + 'a,
{
    type Reader = R;

    // TODO: Calculate normals from vertex data in the case that they're missing?
    fn read_mesh(mut self) -> MeshResult<Mesh<f32>> {
        let mut buffer = String::new();
        self.reader.read_to_string(&mut buffer).unwrap();
        let obj_set = obj::parse(&buffer).unwrap();
        let object = &obj_set.objects[0];
        let mut builder = MeshBuilder::new();
        for element in object.element_set.iter() {
            match element {
                obj::Element::Face(vtn1, vtn2, vtn3) => {
                    let triples = [
                        object.get_vtn_triple(*vtn1).unwrap(),
                        object.get_vtn_triple(*vtn2).unwrap(),
                        object.get_vtn_triple(*vtn3).unwrap(),
                    ];
    
                    let mut primitive = Triangle::default();
                    let mut tex_coord = TextureCoordinates::default();
                    let mut normal = Normals::default();
                 
                    match triples[0] {
                        obj::VTNTriple::V(vp) => {
                            primitive.vertex0 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                            tex_coord[0] = Vector2::zero();
                            normal[0] = Vector3::zero();
                        }
                        obj::VTNTriple::VT(vp, vt) => {
                            primitive.vertex0 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                            tex_coord[0] = Vector2::new(vt.u as f32, vt.v as f32);
                            normal[0] = Vector3::zero();
                        }
                        obj::VTNTriple::VN(vp, vn) => {
                            primitive.vertex0 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                            tex_coord[0] = Vector2::zero();
                            normal[0] = Vector3::new(vn.x as f32, vn.y as f32, vn.z as f32);
                        }
                        obj::VTNTriple::VTN(vp, vt, vn) => {
                            primitive.vertex0 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                            tex_coord[0] = Vector2::new(vt.u as f32, vt.v as f32);
                            normal[0] = Vector3::new(vn.x as f32, vn.y as f32, vn.z as f32);
                        }
                    }
                    match triples[1] {
                        obj::VTNTriple::V(vp) => {
                            primitive.vertex1 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                            tex_coord[1] = Vector2::zero();
                            normal[1] = Vector3::zero();
                        }
                        obj::VTNTriple::VT(vp, vt) => {
                            primitive.vertex1 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                            tex_coord[1] = Vector2::new(vt.u as f32, vt.v as f32);
                            normal[1] = Vector3::zero();
                        }
                        obj::VTNTriple::VN(vp, vn) => {
                            primitive.vertex1 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                            tex_coord[1] = Vector2::zero();
                            normal[1] = Vector3::new(vn.x as f32, vn.y as f32, vn.z as f32);
                        }
                        obj::VTNTriple::VTN(vp, vt, vn) => {
                            primitive.vertex1 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                            tex_coord[1] = Vector2::new(vt.u as f32, vt.v as f32);
                            normal[1] = Vector3::new(vn.x as f32, vn.y as f32, vn.z as f32);
                        }
                    }
                    match triples[2] {
                        obj::VTNTriple::V(vp) => {
                            primitive.vertex2 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                            tex_coord[2] = Vector2::zero();
                            normal[2] = Vector3::zero();
                        }
                        obj::VTNTriple::VT(vp, vt) => {
                            primitive.vertex2 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                            tex_coord[2] = Vector2::new(vt.u as f32, vt.v as f32);
                            normal[2] = Vector3::zero();
                        }
                        obj::VTNTriple::VN(vp, vn) => {
                            primitive.vertex2 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                            tex_coord[2] = Vector2::zero();
                            normal[2] = Vector3::new(vn.x as f32, vn.y as f32, vn.z as f32);
                        }
                        obj::VTNTriple::VTN(vp, vt, vn) => {
                            primitive.vertex2 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                            tex_coord[2] = Vector2::new(vt.u as f32, vt.v as f32);
                            normal[2] = Vector3::new(vn.x as f32, vn.y as f32, vn.z as f32);
                        }
                    }
    
                    builder = builder.with_primitive(primitive, tex_coord, normal);
                }
                _ => {}
            }
        }
    
        let mesh = builder.build();
        
        Ok(mesh)
    }
}

/*
// TODO: Calculate normals from vertex data in the case that they're missing?
pub fn load_mesh(object: &obj::Object) -> Mesh<f32> {
    let mut builder = MeshBuilder::new();
    for element in object.element_set.iter() {
        match element {
            obj::Element::Face(vtn1, vtn2, vtn3) => {
                let triples = [
                    object.get_vtn_triple(*vtn1).unwrap(),
                    object.get_vtn_triple(*vtn2).unwrap(),
                    object.get_vtn_triple(*vtn3).unwrap(),
                ];

                let mut primitive = Triangle::default();
                let mut tex_coord = TextureCoordinates::default();
                let mut normal = Normals::default();
             
                match triples[0] {
                    obj::VTNTriple::V(vp) => {
                        primitive.vertex0 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                        tex_coord[0] = Vector2::zero();
                        normal[0] = Vector3::zero();
                    }
                    obj::VTNTriple::VT(vp, vt) => {
                        primitive.vertex0 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                        tex_coord[0] = Vector2::new(vt.u as f32, vt.v as f32);
                        normal[0] = Vector3::zero();
                    }
                    obj::VTNTriple::VN(vp, vn) => {
                        primitive.vertex0 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                        tex_coord[0] = Vector2::zero();
                        normal[0] = Vector3::new(vn.x as f32, vn.y as f32, vn.z as f32);
                    }
                    obj::VTNTriple::VTN(vp, vt, vn) => {
                        primitive.vertex0 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                        tex_coord[0] = Vector2::new(vt.u as f32, vt.v as f32);
                        normal[0] = Vector3::new(vn.x as f32, vn.y as f32, vn.z as f32);
                    }
                }
                match triples[1] {
                    obj::VTNTriple::V(vp) => {
                        primitive.vertex1 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                        tex_coord[1] = Vector2::zero();
                        normal[1] = Vector3::zero();
                    }
                    obj::VTNTriple::VT(vp, vt) => {
                        primitive.vertex1 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                        tex_coord[1] = Vector2::new(vt.u as f32, vt.v as f32);
                        normal[1] = Vector3::zero();
                    }
                    obj::VTNTriple::VN(vp, vn) => {
                        primitive.vertex1 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                        tex_coord[1] = Vector2::zero();
                        normal[1] = Vector3::new(vn.x as f32, vn.y as f32, vn.z as f32);
                    }
                    obj::VTNTriple::VTN(vp, vt, vn) => {
                        primitive.vertex1 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                        tex_coord[1] = Vector2::new(vt.u as f32, vt.v as f32);
                        normal[1] = Vector3::new(vn.x as f32, vn.y as f32, vn.z as f32);
                    }
                }
                match triples[2] {
                    obj::VTNTriple::V(vp) => {
                        primitive.vertex2 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                        tex_coord[2] = Vector2::zero();
                        normal[2] = Vector3::zero();
                    }
                    obj::VTNTriple::VT(vp, vt) => {
                        primitive.vertex2 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                        tex_coord[2] = Vector2::new(vt.u as f32, vt.v as f32);
                        normal[2] = Vector3::zero();
                    }
                    obj::VTNTriple::VN(vp, vn) => {
                        primitive.vertex2 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                        tex_coord[2] = Vector2::zero();
                        normal[2] = Vector3::new(vn.x as f32, vn.y as f32, vn.z as f32);
                    }
                    obj::VTNTriple::VTN(vp, vt, vn) => {
                        primitive.vertex2 = Vector3::new(vp.x as f32, vp.y as f32, vp.z as f32);
                        tex_coord[2] = Vector2::new(vt.u as f32, vt.v as f32);
                        normal[2] = Vector3::new(vn.x as f32, vn.y as f32, vn.z as f32);
                    }
                }

                builder = builder.with_primitive(primitive, tex_coord, normal);
            }
            _ => {}
        }
    }

    builder.build()
}
*/


