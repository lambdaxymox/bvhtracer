use bvhtracer::{
    Ray,
    Triangle,
    Intersection,
    SurfaceInteraction,
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


fn triangle_intersection_hit(bh: &mut criterion::Criterion) {
    let triangle = Triangle::new(
        Vector3::new(0_f32, 1_f32 / 2_f32, 0_f32),
        Vector3::new(-1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
        Vector3::new(1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
    );
    let ray = gen_hitting_ray();

    bh.bench_function("triangle_intersection_hit", move |bh| bh.iter(|| {
        triangle.intersect(&ray)
    }));
}

fn triangle_intersection_hit_mut(bh: &mut criterion::Criterion) {
    let triangle = Triangle::new(
        Vector3::new(0_f32, 1_f32 / 2_f32, 0_f32),
        Vector3::new(-1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
        Vector3::new(1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
    );
    let ray = gen_hitting_ray();
    let interaction = SurfaceInteraction::new(0_f32, 0_f32, 0_f32);
    let mut intersection = Intersection::from_ray_interaction(ray, interaction);

    bh.bench_function("triangle_intersection_hit_mut", move |bh| bh.iter(|| {
        criterion::black_box(triangle.intersect_mut(&mut intersection))
    }));
}

fn triangle_intersection_miss(bh: &mut criterion::Criterion) {
    let triangle = Triangle::new(
        Vector3::new(0_f32, 1_f32 / 2_f32, 0_f32),
        Vector3::new(-1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
        Vector3::new(1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
    );
    let ray = gen_missing_ray();

    bh.bench_function("triangle_intersection_miss", move |bh| bh.iter(|| {
        triangle.intersect(&ray)
    }));
}

criterion_group!(
    triangle_intersection_benchmarks,
    triangle_intersection_hit,
    triangle_intersection_miss,
    triangle_intersection_hit_mut,
);
criterion_main!(triangle_intersection_benchmarks);

