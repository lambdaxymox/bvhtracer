extern crate bvhtracer;
extern crate cglinalg;
extern crate criterion;
extern crate rand;
extern crate rand_isaac;


use bvhtracer::{
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


fn load_tri_model<P: AsRef<Path>>(path: P) -> Vec<Triangle<f32>> {
    let loaded_tri_data = tri_loader::load(path).unwrap();
    loaded_tri_data.iter().map(|tri| {
        let vertex0 = Vector3::new(tri.vertex0.x, tri.vertex0.y, tri.vertex0.z);
        let vertex1 = Vector3::new(tri.vertex1.x, tri.vertex1.y, tri.vertex1.z);
        let vertex2 = Vector3::new(tri.vertex2.x, tri.vertex2.y, tri.vertex2.z);
        
        Triangle::new(vertex0, vertex1, vertex2)
    }).collect::<Vec<Triangle<_>>>()
}

fn bvh_construction(bh: &mut criterion::Criterion) {
    let mesh = load_tri_model("assets/unity.tri");
    let mut group = bh.benchmark_group("bvh_construction");
    group.sample_size(100);
    group.bench_function("bvh_construction", move |bh| bh.iter(|| {
        let builder = ModelBuilder::new();
        criterion::black_box(builder.with_mesh(mesh.clone()).build())
    }));
    group.finish();
}


criterion_group!(
    bvh_construction_benchmarks,
    bvh_construction,
);
criterion_main!(bvh_construction_benchmarks);

