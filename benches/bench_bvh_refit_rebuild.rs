extern crate bvhtracer;
extern crate cglinalg;
extern crate criterion;
extern crate rand;
extern crate rand_isaac;


use bvhtracer::{
    Scene,
    SceneBuilder,
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


fn load_tri_model<P: AsRef<Path>>(path: P) -> Vec<Triangle<f32>> {
    let loaded_tri_data = tri_loader::load(path).unwrap();
    loaded_tri_data.iter().map(|tri| {
        let vertex0 = Vector3::new(tri.vertex0.x, tri.vertex0.y, tri.vertex0.z);
        let vertex1 = Vector3::new(tri.vertex1.x, tri.vertex1.y, tri.vertex1.z);
        let vertex2 = Vector3::new(tri.vertex2.x, tri.vertex2.y, tri.vertex2.z);
        
        Triangle::new(vertex0, vertex1, vertex2)
    }).collect::<Vec<Triangle<_>>>()
}

fn animate(scene: &mut Scene, r: f32) {
    let a = f32::sin(r) * 0.5;
    for i in 0..scene.objects.len() {
        let o_0 = scene.objects[i].vertex0;
        let s_0 = a * (o_0.y - 0.2) * 0.2;
        let x_0 = o_0.x * f32::cos(s_0) - o_0.y * f32::sin(s_0);
        let y_0 = o_0.x * f32::sin(s_0) + o_0.y * f32::cos(s_0);

        let o_1 = scene.objects[i].vertex1;
        let s_1 = a * (o_1.y - 0.2) * 0.2;
        let x_1 = o_1.x * f32::cos(s_1) - o_1.y * f32::sin(s_1);
        let y_1 = o_1.x * f32::sin(s_1) + o_1.y * f32::cos(s_1);

        let o_2 = scene.objects[i].vertex2;
        let s_2 = a * (o_2.y - 0.2) * 0.2;
        let x_2 = o_2.x * f32::cos(s_2) - o_2.y * f32::sin(s_2);
        let y_2 = o_2.x * f32::sin(s_2) + o_2.y * f32::cos(s_2);

        scene.objects[i] = Triangle::new(
            Vector3::new(x_0, y_0, o_0.z),
            Vector3::new(x_1, y_1, o_1.z),
            Vector3::new(x_2, y_2, o_2.z),
        );
    }
}


fn bvh_rebuild(bh: &mut criterion::Criterion) {
    let original_triangles = load_tri_model("assets/bigben.tri");
    let mut group = bh.benchmark_group("bvh_refit");
    let builder = SceneBuilder::new();
    let mut scene = builder
        .with_objects(original_triangles)
        .build();
    animate(&mut scene, 0.05);
    let animated_triangles = scene.objects;

    group.sample_size(100);
    group.bench_function("bvh_rebuild", move |bh| bh.iter(|| {
        let builder = SceneBuilder::new();
        criterion::black_box(builder.with_objects(animated_triangles.clone()).build())
    }));
    group.finish();
}


fn bvh_refit(bh: &mut criterion::Criterion) {
    let triangles = load_tri_model("assets/bigben.tri");
    let mut group = bh.benchmark_group("bvh_refit");

    let builder = SceneBuilder::new();
    let mut scene = builder
        .with_objects(triangles)
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

