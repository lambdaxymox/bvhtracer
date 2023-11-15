use tiled_array::*;


#[test]
fn test_array_shape_elements_exact_multiple_of_tile_size() {
    const TILE_SIZE: usize = 8;
    let min_capacity_x = 256;
    let min_capacity_y = 128;
    let array: TileArray2D<_, TILE_SIZE> = TileArray2D::with_min_capacity(min_capacity_x, min_capacity_y, 0);
    let expected = (256, 128);
    let result = array.shape_elements();

    assert_eq!(result, expected);
}

#[test]
fn test_array_shape_tiles_exact_multiple_of_tile_size() {
    const TILE_SIZE: usize = 8;
    let min_capacity_x = 256;
    let min_capacity_y = 128;
    let array: TileArray2D<_, TILE_SIZE> = TileArray2D::with_min_capacity(min_capacity_x, min_capacity_y, 0);
    let expected = (32, 16);
    let result = array.shape_tiles();

    assert_eq!(result, expected);
}

#[test]
fn test_array_shape_elements_not_exact_multiple_of_tile_size() {
    const TILE_SIZE: usize = 8;
    let min_capacity_x = 255;
    let min_capacity_y = 127;
    let array: TileArray2D<_, TILE_SIZE> = TileArray2D::with_min_capacity(min_capacity_x, min_capacity_y, 0);
    let expected = (256, 128);
    let result = array.shape_elements();

    assert_eq!(result, expected);
}

#[test]
fn test_array_shape_tiles_not_exact_multiple_of_tile_size() {
    const TILE_SIZE: usize = 8;
    let min_capacity_x = 255;
    let min_capacity_y = 127;
    let array: TileArray2D<_, TILE_SIZE> = TileArray2D::with_min_capacity(min_capacity_x, min_capacity_y, 0);
    let expected = (32, 16);
    let result = array.shape_tiles();

    assert_eq!(result, expected);
}

#[test]
fn test_one_tile_array_indexing() {
    const TILE_SIZE: usize = 8;
    let min_capacity_x = 1;
    let min_capacity_y = 1;
    let mut result: TileArray2D<_, TILE_SIZE> = TileArray2D::with_min_capacity(min_capacity_x, min_capacity_y, (0, 0));
    let (width_elements, height_elements) = result.shape_elements();
    for row in 0..height_elements {
        for col in 0..width_elements {
            result[(row, col)] = (row, col);
        }
    }
    
    for row in 0..height_elements {
        for col in 0..width_elements {
            assert_eq!(result[(row, col)], (row, col));
        }
    }
}

#[test]
fn test_multiple_tile_array_indexing() {
    const TILE_SIZE: usize = 8;
    let min_capacity_x = 16;
    let min_capacity_y = 24;
    let mut result: TileArray2D<_, TILE_SIZE> = TileArray2D::with_min_capacity(min_capacity_x, min_capacity_y, (0, 0));
    let (width_elements, height_elements) = result.shape_elements();
    for row in 0..height_elements {
        for col in 0..width_elements {
            result[(row, col)] = (row, col);
        }
    }
    
    for row in 0..height_elements {
        for col in 0..width_elements {
            assert_eq!(result[(row, col)], (row, col));
        }
    }
}

