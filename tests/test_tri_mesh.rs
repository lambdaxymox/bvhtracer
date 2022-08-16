extern crate bvhtracer;
extern crate cglinalg;


use bvhtracer::{
    Mesh,
    MeshBuilder,
    Triangle,
    Normals,
    TextureCoordinates,
    TriMeshDecoder,
    MeshDecoder,
};
use cglinalg::{
    Vector2,
    Vector3,  
};
use std::fs::{
    File,
};
use std::io::{
    Cursor,
};


fn test_case() -> &'static str {
    r"                                                                                       \
    0.577350 -0.500000 -0.100000 0.577350 -0.500000  0.100000 -0.577350 -0.500000  0.100000  \
    -0.577350 -0.500000 -0.100000 0.000000  0.500000 -0.100000  0.000000  0.500000  0.100000 \
    "
}

fn mesh() -> Mesh<f32> {
    MeshBuilder::new()
        .with_primitive(
            Triangle::new(
                Vector3::new( 0.577350, -0.500000, -0.100000), 
                Vector3::new( 0.577350, -0.500000,  0.100000), 
                Vector3::new(-0.577350, -0.500000,  0.100000)
            ),
            TextureCoordinates::from([
                Vector2::zero(), 
                Vector2::zero(), 
                Vector2::zero()]
            ),
            Normals::from([
                Vector3::new(0.0, 1.0, 0.0), 
                Vector3::new(0.0, 1.0, 0.0), 
                Vector3::new(0.0, 1.0, 0.0)
            ]),
        )
        .with_primitive(
            Triangle::new(
                Vector3::new(-0.577350, -0.500000, -0.100000), 
                Vector3::new( 0.000000,  0.500000, -0.100000), 
                Vector3::new( 0.000000,  0.500000,  0.100000)
            ),
            TextureCoordinates::from([
                Vector2::zero(), 
                Vector2::zero(), 
                Vector2::zero()
            ]),
            Normals::from([
                Vector3::new(-0.86602545, 0.49999988, -1.7462564e-7), 
                Vector3::new(-0.86602545, 0.49999988, -1.7462564e-7), 
                Vector3::new(-0.86602545, 0.49999988, -1.7462564e-7),
            ]),
        )
        .build()
}

#[test]
fn test_tri_mesh_decoder1() {
    let expected = mesh();
    let test_case = test_case();
    let reader = Cursor::new(test_case);
    let result = TriMeshDecoder::new(reader)
        .read_mesh()
        .unwrap();

    assert_eq!(result, expected);
}

#[test]
fn test_tri_mesh_decoder2() {
    let expected = mesh();
    let reader = File::open("assets/triangle.tri").unwrap();
    let result = TriMeshDecoder::new(reader)
        .read_mesh()
        .unwrap();

    assert_eq!(result, expected);
}

