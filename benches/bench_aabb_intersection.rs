extern crate bvhtracer;
extern crate cglinalg;
extern crate criterion;
extern crate rand;
extern crate rand_isaac;


use bvhtracer::{
    Ray,
    Aabb,
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

fn aabb_intersection_hit(bh: &mut criterion::Criterion) {
    let aabb = Aabb::new(
        Vector3::new(-1_f32, -1_f32, -1_f32), 
        Vector3::new(1_f32, 1_f32, 1_f32)
    );
    let ray = gen_hitting_ray();

    bh.bench_function("aabb_intersection_hit", move |bh| bh.iter(|| {
        criterion::black_box(aabb.intersect(&ray))
    }));
}

fn aabb_intersection_miss(bh: &mut criterion::Criterion) {
    let aabb = Aabb::new(
        Vector3::new(-1_f32, -1_f32, -1_f32), 
        Vector3::new(1_f32, 1_f32, 1_f32)
    );
    let ray = gen_missing_ray();

    bh.bench_function("aabb_intersection_miss", move |bh| bh.iter(|| {
        criterion::black_box(aabb.intersect(&ray))
    }));
}

criterion_group!(
    aabb_intersection_benchmarks,
    aabb_intersection_hit,
    aabb_intersection_miss,
);
criterion_main!(aabb_intersection_benchmarks);

