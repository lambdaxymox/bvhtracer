use bvhtracer::{
    Mesh,
    MeshBuilder,
    Triangle,
    Normals,
    TextureCoordinates,
    ObjMeshDecoder,
    MeshDecoder,
};
use cglinalg::{
    Vector2,
    Vector3,  
};
use std::io::{
    Cursor,
};


fn test_case() -> String {
    // The texture coordinates in this test case should not be used outside of it.
    // They are added in to test the obj mesh parser is correct and are probably not
    // a coherent set of texture coordinates on a real cube map.
    let obj_file = String::from(r"\
        # cube.obj                  \
        #                           \
                                    \
        g cube                      \
                                    \
        v  0.0  0.0  0.0            \
        v  0.0  0.0  1.0            \
        v  0.0  1.0  0.0            \
        v  0.0  1.0  1.0            \
        v  1.0  0.0  0.0            \
        v  1.0  0.0  1.0            \
        v  1.0  1.0  0.0            \
        v  1.0  1.0  1.0            \
                                    \
        vt  0.0   0.66666666        \
        vt  0.25  0.66666666        \
        vt  0.0   0.33333333        \
        vt  0.25  1.0               \
        vt  0.5   1.0               \
        vt  0.5   0.66666666        \
        vt  0.25  0.33333333        \
        vt  0.50  0.33333333        \
        vt  0.75  0.66666666        \
        vt  0.75  0.33333333        \
        vt  1.0   0.66666666        \
        vt  1.0   0.33333333        \
        vt  0.5   0.0               \
        vt  0.25  0.0               \
                                    \
        vn  0.0  0.0  1.0           \
        vn  0.0  0.0 -1.0           \
        vn  0.0  1.0  0.0           \
        vn  0.0 -1.0  0.0           \
        vn  1.0  0.0  0.0           \
        vn -1.0  0.0  0.0           \
                                    \
        f  1/2/2   7/3/2   5/1/2    \
        f  1/2/2   3/7/2   7/3/2    \
        f  1/5/6   4/2/6   3/4/6    \
        f  1/5/6   2/6/6   4/2/6    \
        f  3/6/3   8/7/3   7/2/3    \
        f  3/6/3   4/8/3   8/7/3    \
        f  5/8/5   7/14/5  8/7/5    \
        f  5/8/5   8/13/5  6/14/5   \
        f  1/9/4   5/8/4   6/6/4    \
        f  1/9/4   6/10/4  2/8/4    \
        f  2/11/1  6/10/1  8/9/1    \
        f  2/11/1  8/12/1  4/10/1   \
    ");

    obj_file
}

fn mesh() -> Mesh<f32> {
    let builder = MeshBuilder::new()
        .with_primitive(
            Triangle::new(
                Vector3::new(0_f32, 0_f32, 0_f32), 
                Vector3::new(1_f32, 1_f32, 0_f32), 
                Vector3::new(1_f32, 0_f32, 0_f32),
            ),
            TextureCoordinates::from([
                Vector2::new(0.25, 0.66666666), 
                Vector2::new(0.0, 0.33333333), 
                Vector2::new(0.0, 0.66666666),
            ]),
            Normals::from([
                Vector3::new(0_f32, 0_f32, -1_f32),
                Vector3::new(0_f32, 0_f32, -1_f32),
                Vector3::new(0_f32, 0_f32, -1_f32),
            ]),
        )
        .with_primitive(
            Triangle::new(
                Vector3::new(0_f32, 0_f32, 0_f32),
                Vector3::new(0_f32, 1_f32, 0_f32),
                Vector3::new(1_f32, 1_f32, 0_f32),
            ),
            TextureCoordinates::from([
                Vector2::new(0.25, 0.66666666), 
                Vector2::new(0.25, 0.33333333), 
                Vector2::new(0.0, 0.33333333),
            ]),
            Normals::from([
                Vector3::new(0_f32, 0_f32, -1_f32),
                Vector3::new(0_f32, 0_f32, -1_f32),
                Vector3::new(0_f32, 0_f32, -1_f32),
            ]),
        )
        .with_primitive(
            Triangle::new(
                Vector3::new(0_f32, 0_f32, 0_f32), 
                Vector3::new(0_f32, 1_f32, 1_f32), 
                Vector3::new(0_f32, 1_f32, 0_f32),
            ),
            TextureCoordinates::from([
                Vector2::new(0.5, 1.0), 
                Vector2::new(0.25, 0.66666666), 
                Vector2::new(0.25, 1.0),
            ]),
            Normals::from([
                Vector3::new(-1_f32, 0_f32, 0_f32),
                Vector3::new(-1_f32, 0_f32, 0_f32),
                Vector3::new(-1_f32, 0_f32, 0_f32),
            ]),
        )
        .with_primitive(
            Triangle::new(
                Vector3::new(0_f32, 0_f32, 0_f32),
                Vector3::new(0_f32, 0_f32, 1_f32),
                Vector3::new(0_f32, 1_f32, 1_f32),
            ),
            TextureCoordinates::from([
                Vector2::new(0.5, 1.0), 
                Vector2::new(0.5, 0.66666666), 
                Vector2::new(0.25, 0.66666666),
            ]),
            Normals::from([
                Vector3::new(-1_f32, 0_f32, 0_f32),
                Vector3::new(-1_f32, 0_f32, 0_f32),
                Vector3::new(-1_f32, 0_f32, 0_f32),
            ]),
        )
        .with_primitive(
            Triangle::new(
                Vector3::new(0_f32, 1_f32, 0_f32), 
                Vector3::new(1_f32, 1_f32, 1_f32), 
                Vector3::new(1_f32, 1_f32, 0_f32),
            ),
            TextureCoordinates::from([
                Vector2::new(0.5, 0.66666666), 
                Vector2::new(0.25, 0.33333333), 
                Vector2::new(0.25, 0.66666666),
            ]),
            Normals::from([
                Vector3::new(0_f32, 1_f32, 0_f32),
                Vector3::new(0_f32, 1_f32, 0_f32),
                Vector3::new(0_f32, 1_f32, 0_f32),
            ]),
        )
        .with_primitive(
            Triangle::new(
                Vector3::new(0_f32, 1_f32, 0_f32),
                Vector3::new(0_f32, 1_f32, 1_f32),
                Vector3::new(1_f32, 1_f32, 1_f32),
            ),
            TextureCoordinates::from([
                Vector2::new(0.5, 0.66666666), 
                Vector2::new(0.5, 0.33333333), 
                Vector2::new(0.25, 0.33333333),
            ]),
            Normals::from([
                Vector3::new(0_f32, 1_f32, 0_f32),
                Vector3::new(0_f32, 1_f32, 0_f32),
                Vector3::new(0_f32, 1_f32, 0_f32),
            ]),
        )
        .with_primitive(
            Triangle::new(
                Vector3::new(1_f32, 0_f32, 0_f32), 
                Vector3::new(1_f32, 1_f32, 0_f32), 
                Vector3::new(1_f32, 1_f32, 1_f32),
            ),
            TextureCoordinates::from([
                Vector2::new(0.5, 0.33333333), 
                Vector2::new(0.25, 0.0), 
                Vector2::new(0.25, 0.33333333),
            ]),
            Normals::from([
                Vector3::new(1_f32, 0_f32, 0_f32),
                Vector3::new(1_f32, 0_f32, 0_f32),
                Vector3::new(1_f32, 0_f32, 0_f32),
            ]),
        )
        .with_primitive(
            Triangle::new(
                Vector3::new(1_f32, 0_f32, 0_f32),
                Vector3::new(1_f32, 1_f32, 1_f32),
                Vector3::new(1_f32, 0_f32, 1_f32),
            ),
            TextureCoordinates::from([
                Vector2::new(0.5, 0.33333333), 
                Vector2::new(0.5, 0.0), 
                Vector2::new(0.25, 0.0),
            ]),
            Normals::from([
                Vector3::new(1_f32, 0_f32, 0_f32),
                Vector3::new(1_f32, 0_f32, 0_f32),
                Vector3::new(1_f32, 0_f32, 0_f32),
            ]),
        )
        .with_primitive(
            Triangle::new(
                Vector3::new(0_f32, 0_f32, 0_f32), 
                Vector3::new(1_f32, 0_f32, 0_f32), 
                Vector3::new(1_f32, 0_f32, 1_f32),
            ),
            TextureCoordinates::from([
                Vector2::new(0.75, 0.66666666), 
                Vector2::new(0.5, 0.33333333), 
                Vector2::new(0.5, 0.66666666),
            ]),
            Normals::from([
                Vector3::new(0_f32, -1_f32, 0_f32),
                Vector3::new(0_f32, -1_f32, 0_f32),
                Vector3::new(0_f32, -1_f32, 0_f32),
            ]),
        )
        .with_primitive(
            Triangle::new(
                Vector3::new(0_f32, 0_f32, 0_f32),
                Vector3::new(1_f32, 0_f32, 1_f32),
                Vector3::new(0_f32, 0_f32, 1_f32),
            ),
            TextureCoordinates::from([
                Vector2::new(0.75, 0.66666666), 
                Vector2::new(0.75, 0.33333333), 
                Vector2::new(0.5, 0.33333333),
            ]),
            Normals::from([
                Vector3::new(0_f32, -1_f32, 0_f32),
                Vector3::new(0_f32, -1_f32, 0_f32),
                Vector3::new(0_f32, -1_f32, 0_f32),
            ]),
        )
        .with_primitive(
            Triangle::new(
                Vector3::new(0_f32, 0_f32, 1_f32), 
                Vector3::new(1_f32, 0_f32, 1_f32), 
                Vector3::new(1_f32, 1_f32, 1_f32),
            ),
            TextureCoordinates::from([
                Vector2::new(1.0, 0.66666666), 
                Vector2::new(0.75, 0.33333333), 
                Vector2::new(0.75, 0.66666666),
            ]),
            Normals::from([
                Vector3::new(0_f32, 0_f32, 1_f32),
                Vector3::new(0_f32, 0_f32, 1_f32),
                Vector3::new(0_f32, 0_f32, 1_f32),
            ]),
        )
        .with_primitive(
            Triangle::new(
                Vector3::new(0_f32, 0_f32, 1_f32),
                Vector3::new(1_f32, 1_f32, 1_f32),
                Vector3::new(0_f32, 1_f32, 1_f32),
            ),
            TextureCoordinates::from([
                Vector2::new(1.0, 0.66666666),
                Vector2::new(1.0, 0.33333333), 
                Vector2::new(0.75, 0.33333333),
            ]),
            Normals::from([
                Vector3::new(0_f32, 0_f32, 1_f32),
                Vector3::new(0_f32, 0_f32, 1_f32),
                Vector3::new(0_f32, 0_f32, 1_f32),
            ]),
        );
    
    builder.build()
}

#[test]
fn test_obj_mesh_decoder() {
    let expected = mesh();
    let test_case = test_case();
    let reader = Cursor::new(test_case);
    let result = ObjMeshDecoder::new(reader)
        .read_mesh()
        .unwrap();

    assert_eq!(result, expected);
}


#[test]
fn test_obj_mesh_decoder_primitives() {
    let expected_mesh = mesh();
    let expected = expected_mesh.primitives();
    let test_case = test_case();
    let reader = Cursor::new(test_case);
    let result_mesh = ObjMeshDecoder::new(reader)
        .read_mesh()
        .unwrap();
    let result = result_mesh.primitives();

    assert_eq!(result, expected);
} 

#[test]
fn test_obj_mesh_decoder_tex_coords() {
    let expected_mesh = mesh();
    let expected = expected_mesh.tex_coords();
    let test_case = test_case();
    let reader = Cursor::new(test_case);
    let result_mesh = ObjMeshDecoder::new(reader)
        .read_mesh()
        .unwrap();
    let result = result_mesh.tex_coords();

    assert_eq!(result, expected);
}

#[test]
fn test_obj_mesh_decoder_normals() {
    let expected_mesh = mesh();
    let expected = expected_mesh.normals();
    let test_case = test_case();
    let reader = Cursor::new(test_case);
    let result_mesh = ObjMeshDecoder::new(reader)
        .read_mesh()
        .unwrap();
    let result = result_mesh.normals();

    assert_eq!(result, expected);
}

#[test]
fn test_obj_mesh_decoder_primitives_elementwise() {
    let expected_mesh = mesh();
    let expected = expected_mesh.primitives();
    let test_case = test_case();
    let reader = Cursor::new(test_case);
    let result_mesh = ObjMeshDecoder::new(reader)
        .read_mesh()
        .unwrap();
    let result = result_mesh.primitives();

    assert_eq!(result.len(), expected.len());

    for i in 0..result.len() {
        assert_eq!(result[i], expected[i]);
    }
} 

#[test]
fn test_obj_mesh_decoder_tex_coords_elementwise() {
    let expected_mesh = mesh();
    let expected = expected_mesh.tex_coords();
    let test_case = test_case();
    let reader = Cursor::new(test_case);
    let result_mesh = ObjMeshDecoder::new(reader)
        .read_mesh()
        .unwrap();
    let result = result_mesh.tex_coords();

    assert_eq!(result.len(), expected.len());

    for i in 0..result.len() {
        assert_eq!(result[i], expected[i]);
    }
}

#[test]
fn test_obj_mesh_decoder_normals_elementwise() {
    let expected_mesh = mesh();
    let expected = expected_mesh.normals();
    let test_case = test_case();
    let reader = Cursor::new(test_case);
    let result_mesh = ObjMeshDecoder::new(reader)
        .read_mesh()
        .unwrap();
    let result = result_mesh.normals();

    assert_eq!(result.len(), expected.len());

    for i in 0..result.len() {
        assert_eq!(result[i], expected[i]);
    }
}

