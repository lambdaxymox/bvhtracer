extern crate bvhtracer;
extern crate cglinalg;
extern crate criterion;
extern crate rand;
extern crate rand_isaac;


use bvhtracer::{
    Ray,
    ModelInstance,
    ModelBuilder,
    Triangle,
};
use cglinalg::{
    Magnitude,
    Vector3,
};
use rand::{
    Rng, 
};
use rand_isaac::{
    IsaacRng,
};
use criterion::{
    criterion_group,
    criterion_main,
};


const PI: f32 = std::f32::consts::PI;


fn create_mesh_sphere(x_segments: u32, y_segments: u32) -> Vec<Triangle<f32>> {
    let mut vertices = vec![];
    for y in 0..(y_segments + 1) {
        for x in 0..(x_segments + 1) {
            let x_segment = x as f32 / x_segments as f32;
            let y_segment = y as f32 / y_segments as f32;
            let x_pos = f32::cos(x_segment * 2_f32 * PI) * f32::sin(y_segment * PI);
            let y_pos = f32::cos(y_segment * PI);
            let z_pos = f32::sin(x_segment * 2_f32 * PI) * f32::sin(y_segment * PI);

            vertices.push(Vector3::new(x_pos, y_pos, z_pos));
        }
    }

    let mut mesh = vec![];
    for chunk in vertices.chunks(3) {
        mesh.push(Triangle::new(chunk[0], chunk[1], chunk[2]));
    }

    mesh
}

fn scene() -> ModelInstance {
    let mesh = create_mesh_sphere(50, 50);
    let builder = ModelBuilder::new();
    
    builder.with_primitives(mesh).build()
}

fn sample_unit_sphere(rng: &mut IsaacRng) -> Vector3<f32> {
    loop {
        let a = rng.gen::<f32>();
        let b = rng.gen::<f32>();
        let c = rng.gen::<f32>();
        let p = Vector3::new(a, b, c) * 2_f32 - Vector3::new(1_f32, 1_f32, 1_f32);

        // If the sample falls inside the unit sphere, we can return.
        if p.magnitude() < 1.0 {
            return p;
        }
    }
}

fn gen_hitting_ray() -> Ray<f32> {
    use rand::SeedableRng;
    let mut rng = IsaacRng::seed_from_u64(0);
    let sphere_radius = 10_f32;
    let sample = sample_unit_sphere(&mut rng);
    let ray_direction = -sample;
    let ray_origin = sample * sphere_radius;

    Ray::from_origin_dir(ray_origin, ray_direction)
}

fn gen_missing_ray() -> Ray<f32> {
    use rand::SeedableRng;
    let mut rng = IsaacRng::seed_from_u64(0);
    let sphere_radius = 10_f32;
    let sample = sample_unit_sphere(&mut rng);
    let ray_direction = sample;
    let ray_origin = sample * sphere_radius;

    Ray::from_origin_dir(ray_origin, ray_direction)
}

fn bvh_intersection_hit(bh: &mut criterion::Criterion) {
    let scene = scene();
    let ray = gen_hitting_ray();

    bh.bench_function("bvh_intersection_hit", move |bh| bh.iter(|| {
        scene.intersect(&ray)
    }));
}

fn bvh_intersection_miss(bh: &mut criterion::Criterion) {
    let scene = scene();
    let ray = gen_missing_ray();

    bh.bench_function("bvh_intersection_miss", move |bh| bh.iter(|| {
        scene.intersect(&ray)
    }));
}


criterion_group!(
    bvh_intersection_benchmarks,
    bvh_intersection_hit,
    bvh_intersection_miss,
);
criterion_main!(bvh_intersection_benchmarks);

