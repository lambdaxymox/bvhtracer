use bvhtracer::{
    TextureCoordinates,
    Normals,
    MeshBuilder,
    ModelInstance,
    ModelBuilder,
    Triangle,
    Ray,
};
use cglinalg::{
    Magnitude,
    Vector3,
};


fn scene() -> ModelInstance {
    let displacement = Vector3::new(0_f32, 10_f32, 0_f32);
    let triangle1 = Triangle::new(
        Vector3::new(0_f32, 1_f32 / 2_f32, 0_f32),
        Vector3::new(-1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
        Vector3::new(1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
    );
    let tex_coords1 = TextureCoordinates::default();
    let normals1 = Normals::default();
    let triangle2 = Triangle::new(
        triangle1.vertices[0] - displacement,
        triangle1.vertices[1] - displacement,
        triangle1.vertices[2] - displacement
    );
    let tex_coords2 = TextureCoordinates::default();
    let normals2 = Normals::default();
    let triangle3 = Triangle::new(
        triangle1.vertices[0] + displacement,
        triangle1.vertices[1] + displacement,
        triangle1.vertices[2] + displacement
    );
    let tex_coords3 = TextureCoordinates::default();
    let normals3 = Normals::default();
    let mesh = MeshBuilder::new()
        .with_primitive(triangle1, tex_coords1, normals1)
        .with_primitive(triangle2, tex_coords2, normals2)
        .with_primitive(triangle3, tex_coords3, normals3)
        .build();
    let builder = ModelBuilder::new();
    
    builder.with_mesh(mesh).build()
}

#[test]
fn test_three_triangles_centroids_hit() {
    let scene = scene();
    let mesh = scene.model();
    for triangle in mesh.borrow().primitives().iter() {
        let ray_origin = Vector3::new(0_f32, 0_f32, 10_f32);
        let ray_direction = (triangle.centroid() - ray_origin).normalize();
        let ray = Ray::from_origin_dir(ray_origin, ray_direction);

        assert!(triangle.intersect(&ray).is_some());
    }
}
