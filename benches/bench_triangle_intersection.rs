extern crate bvhtracer;
extern crate cglinalg;
extern crate criterion;
extern crate rand;
extern crate rand_isaac;

use bvhtracer::{
    Ray,
    Triangle,
};
use cglinalg::{
    Magnitude,
    Vector3,
};
use rand::{
    Rng, 
    prelude::Distribution,
    distributions::Standard,
};

use rand_isaac::{
    IsaacRng,
};

use criterion::{
    criterion_group,
    criterion_main,
};

fn intersect_tri(ray: &Ray<f32>, tri: &Triangle<f32>) -> f32 {
	let edge1 = tri.vertex1 - tri.vertex0;
	let edge2 = tri.vertex2 - tri.vertex0;
	let h = ray.direction.cross(&edge2);
	let a = edge1.dot(&h);
	if a > -0.0001_f32 && a < 0.0001_f32 {
        return f32::MAX; // ray parallel to triangle
    }
	let f = 1_f32 / a;
	let s = ray.origin - tri.vertex0;
	let u = f * s.dot(&h);
	if u < 0_f32 || u > 1_f32 {
        return f32::MAX;
    }
	let q = s.cross(&edge1);
	let v = f * ray.direction.dot(&q);
	if v < 0_f32 || u + v > 1_f32 {
        return f32::MAX;
    }
	let t = f * edge2.dot(&q);
	if t > 0.0001_f32 {
        return f32::min(ray.t, t);
    } else {
        return f32::MAX;
    }
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

fn triangle_intersection_hit2(bh: &mut criterion::Criterion) {
    let triangle = Triangle::new(
        Vector3::new(0_f32, 1_f32 / 2_f32, 0_f32),
        Vector3::new(-1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
        Vector3::new(1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
    );
    let ray = gen_hitting_ray();

    bh.bench_function("triangle_intersection_hit2", move |bh| bh.iter(|| {
        intersect_tri(&ray, &triangle)
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
    triangle_intersection_hit2,
);
criterion_main!(triangle_intersection_benchmarks);

