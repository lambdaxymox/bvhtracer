use bvhtracer::{
    Mesh,
    MeshBuilder,
    TextureCoordinates,
    Normals,
    ModelInstance,
    ModelBuilder,
    Triangle,
};
use cglinalg::{
    Vector3,
};
use criterion::{
    criterion_group,
    criterion_main,
};
use std::path::{
    Path,
};


fn load_tri_model<P: AsRef<Path>>(path: P) -> Mesh<f32> {
    let loaded_tri_data = tri_loader::load(path).unwrap();
    loaded_tri_data.iter().fold(MeshBuilder::new(), |builder, tri| {
        let vertex0 = Vector3::new(tri.vertex0.x, tri.vertex0.y, tri.vertex0.z);
        let vertex1 = Vector3::new(tri.vertex1.x, tri.vertex1.y, tri.vertex1.z);
        let vertex2 = Vector3::new(tri.vertex2.x, tri.vertex2.y, tri.vertex2.z);
        let primitive = Triangle::new(vertex0, vertex1, vertex2);
        let tex_coords = TextureCoordinates::default();
        let normals = Normals::default();

        builder.with_primitive(primitive, tex_coords, normals)
    })
    .build()
}

fn animate(scene: &mut ModelInstance, r: f32) {
    let a = f32::sin(r) * 0.5;
    let mesh = scene.model();
    let primitive_count = mesh.borrow().len_primitives();
    for i in 0..primitive_count {
        let o_0 = mesh.borrow().primitives()[i].vertices[0];
        let s_0 = a * (o_0.y - 0.2) * 0.2;
        let x_0 = o_0.x * f32::cos(s_0) - o_0.y * f32::sin(s_0);
        let y_0 = o_0.x * f32::sin(s_0) + o_0.y * f32::cos(s_0);

        let o_1 = mesh.borrow().primitives()[i].vertices[1];
        let s_1 = a * (o_1.y - 0.2) * 0.2;
        let x_1 = o_1.x * f32::cos(s_1) - o_1.y * f32::sin(s_1);
        let y_1 = o_1.x * f32::sin(s_1) + o_1.y * f32::cos(s_1);

        let o_2 = mesh.borrow().primitives()[i].vertices[2];
        let s_2 = a * (o_2.y - 0.2) * 0.2;
        let x_2 = o_2.x * f32::cos(s_2) - o_2.y * f32::sin(s_2);
        let y_2 = o_2.x * f32::sin(s_2) + o_2.y * f32::cos(s_2);

        mesh.borrow_mut().primitives_mut()[i] = Triangle::new(
            Vector3::new(x_0, y_0, o_0.z),
            Vector3::new(x_1, y_1, o_1.z),
            Vector3::new(x_2, y_2, o_2.z),
        );
    }
}


fn bvh_rebuild(bh: &mut criterion::Criterion) {
    let original_mesh = load_tri_model("assets/bigben.tri");
    let mut group = bh.benchmark_group("bvh_refit");
    let builder = ModelBuilder::new();
    let mut scene = builder
        .with_mesh(original_mesh)
        .build();
    animate(&mut scene, 0.05);
    let animated_mesh = scene.model()
        .borrow()
        .mesh()
        .clone();

    group.sample_size(100);
    group.bench_function("bvh_rebuild", move |bh| bh.iter(|| {
        let builder = ModelBuilder::new();
        criterion::black_box(builder.with_mesh(animated_mesh.clone()).build())
    }));
    group.finish();
}


fn bvh_refit(bh: &mut criterion::Criterion) {
    let mesh = load_tri_model("assets/bigben.tri");
    let mut group = bh.benchmark_group("bvh_refit");

    let builder = ModelBuilder::new();
    let mut scene = builder
        .with_mesh(mesh)
        .build();
    animate(&mut scene, 0.05);

    group.sample_size(100);
    group.bench_function("bvh_refit", move |bh| bh.iter(|| {
        criterion::black_box(scene.refit())
    }));
    group.finish();
}


criterion_group!(
    bvh_refit_rebuild_benchmarks,
    bvh_rebuild,
    bvh_refit,
);
criterion_main!(bvh_refit_rebuild_benchmarks);

