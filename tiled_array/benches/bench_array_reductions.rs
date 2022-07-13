extern crate tiled_array;
extern crate criterion;
extern crate rand;
extern crate rand_isaac;


use tiled_array::{
    TiledArray2D,
};
use criterion::{
    criterion_group,
    criterion_main,
};

const TILE_SIZE: usize = 8;

fn sum_vec(vec: &Vec<u32>) -> u32 {
    let mut total = 0;
    for elem in vec.iter() {
        total += elem;
    }

    total
}

fn sum_array(array: &TiledArray2D<u32, TILE_SIZE>) -> u32 {
    let mut total = 0;
    for tile in array.tile_iter() {
        for i in 0..TILE_SIZE {
            for j in 0..TILE_SIZE {
                total += tile[i][j];
            }
        }
    }

    total
}

fn tiled_array_reduction_vec(bh: &mut criterion::Criterion) {
    let width = 1024;
    let height = 1024;
    let mut vec = vec![0_u32; width * height];
    for i in 0..width {
        for j in 0..height {
            vec[height * j + i] = (i * j) as u32;
        }
    }

    bh.bench_function("tiled_array_reduction_vec", move |bh| bh.iter(|| {
        sum_vec(&vec)
    }));
}

fn tiled_array_reduction_tiled_array(bh: &mut criterion::Criterion) {
    let mut array: TiledArray2D<_, TILE_SIZE> = TiledArray2D::with_min_capacity(1024, 1024, 0_u32);
    let (width, height) = array.shape_elements();
    for i in 0..width {
        for j in 0..height {
            array[(i, j)] = (i * j) as u32;
        }
    }

    bh.bench_function("tiled_array_reduction_tiled_array", move |bh| bh.iter(|| {
        sum_array(&array)
    }));
}


criterion_group!(
    tiled_array_reduction_benchmarks,
    tiled_array_reduction_vec,
    tiled_array_reduction_tiled_array,
);
criterion_main!(tiled_array_reduction_benchmarks);

