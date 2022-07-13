use std::ops;


type Tile<T, const TILE_SIZE: usize> = [[T; TILE_SIZE]; TILE_SIZE];

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct TileIndex(usize, usize);


#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
struct TileEntry {
    index: usize,
    offset: usize,
}

impl TileEntry {
    fn new(index: usize, offset: usize) -> Self {
        Self { index, offset }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct TiledArray2D<T, const TILE_SIZE: usize> {
    tiles_x: Vec<TileEntry>,
    tiles_y: Vec<TileEntry>,
    tile_width: usize,
    tile_height: usize,
    data: Vec<Tile<T, TILE_SIZE>>,
}

impl<T, const TILE_SIZE: usize> TiledArray2D<T, TILE_SIZE> 
where
    T: Copy,
{
    pub fn with_min_capacity(min_capacity_x: usize, min_capacity_y: usize, default_value: T) -> Self {
        let tile_count_x = min_capacity_x / TILE_SIZE;
        let tile_count_y = min_capacity_y / TILE_SIZE;
        let remainder_x = min_capacity_x % TILE_SIZE;
        let remainder_y = min_capacity_y % TILE_SIZE;
        let padded_tile_count_x = if remainder_x != 0 { tile_count_x + 1 } else { tile_count_x };
        let padded_tile_count_y = if remainder_y != 0 { tile_count_y + 1 } else { tile_count_y };
        let tile_element_count = TILE_SIZE * TILE_SIZE;
        // let padded_capacity = padded_tile_count_x * padded_tile_count_y * tile_element_count;
        let padded_capacity = padded_tile_count_x * padded_tile_count_y;
        let padded_element_width = padded_tile_count_x * TILE_SIZE;
        let padded_element_height = padded_tile_count_y * TILE_SIZE;

        let mut tiles_x = vec![];
        for i in 0..padded_element_width {
            let index = i / TILE_SIZE;
            let offset = i % TILE_SIZE;
            let entry = TileEntry::new(index, offset);
            
            tiles_x.push(entry);
        }

        let mut tiles_y = vec![];
        for j in 0..padded_element_height {
            let index = j / TILE_SIZE;
            let offset = j % TILE_SIZE;
            let entry = TileEntry::new(index, offset);
            
            tiles_y.push(entry);
        }
        
        let default_array = [[default_value; TILE_SIZE]; TILE_SIZE];
        let data = vec![default_array; padded_capacity];

        Self { 
            tiles_x, 
            tiles_y, 
            tile_width: padded_tile_count_x, 
            tile_height: padded_tile_count_y, 
            data 
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        <Self as AsRef<[T]>>::as_ref(self)
    }

    #[inline]
    pub fn shape_elements(&self) -> (usize, usize) {
        (self.tile_width * TILE_SIZE, self.tile_height * TILE_SIZE)
    }

    #[inline]
    pub fn shape_tiles(&self) -> (usize, usize) {
        (self.tile_width, self.tile_height)
    }

    #[inline]
    pub fn width_elements(&self) -> usize {
        self.tile_width * TILE_SIZE
    }

    #[inline]
    pub fn height_elements(&self) -> usize {
        self.tile_height * TILE_SIZE
    }

    #[inline]
    fn len_elements(&self) -> usize {
        self.data.len() * TILE_SIZE * TILE_SIZE
    }

    #[inline]
    fn len_tiles(&self) -> usize {
        self.data.len()
    }
}

impl<T, const TILE_SIZE: usize> AsRef<[T]> for TiledArray2D<T, TILE_SIZE> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        use std::ptr;
        unsafe {
            let base_ptr = self.data.as_ptr() as *const T;
            let len_tiles = self.data.len();
            let elements_per_tile = TILE_SIZE * TILE_SIZE;
            let len_elements = len_tiles * elements_per_tile;
            let slice = ptr::slice_from_raw_parts(base_ptr, len_elements);

            &*(slice)
        }
    }
}

impl<T, const TILE_SIZE: usize> ops::Index<(usize, usize)> for TiledArray2D<T, TILE_SIZE>
where
    T: Copy
{
    type Output = T;
    
    #[inline]
    fn index(&self, _index: (usize, usize)) -> &Self::Output {
        let tile_x = self.tiles_x[_index.1];
        let tile_y = self.tiles_y[_index.0];
        let tile_index = self.tile_width * tile_y.index + tile_x.index;

        &self.data[tile_index][tile_y.offset][tile_x.offset]
    }
}

impl<T, const TILE_SIZE: usize> ops::IndexMut<(usize, usize)> for TiledArray2D<T, TILE_SIZE>
where
    T: Copy
{   
    #[inline]
    fn index_mut(&mut self, _index: (usize, usize)) -> &mut Self::Output {
        let tile_x = self.tiles_x[_index.1];
        let tile_y = self.tiles_y[_index.0];
        let tile_index = self.tile_width * tile_y.index + tile_x.index;

        &mut self.data[tile_index][tile_y.offset][tile_x.offset]
    }
}

impl<T, const TILE_SIZE: usize> ops::Index<TileIndex> for TiledArray2D<T, TILE_SIZE>
where
    T: Copy
{
    type Output = Tile<T, TILE_SIZE>;
    
    #[inline]
    fn index(&self, _index: TileIndex) -> &Self::Output {
        let tile_index = self.tile_width * _index.1 + _index.0;

        &self.data[tile_index]
    }
}

impl<T, const TILE_SIZE: usize> ops::IndexMut<TileIndex> for TiledArray2D<T, TILE_SIZE>
where
    T: Copy
{   
    #[inline]
    fn index_mut(&mut self, _index: TileIndex) -> &mut Self::Output {
        let tile_index = self.tile_width * _index.1 + _index.0;

        &mut self.data[tile_index]
    }
}


#[cfg(test)]
mod tests {
    use super::{
        TileIndex,
        TiledArray2D,
    };


    #[test]
    fn test_one_tile_array_storage() {
        const TILE_SIZE: usize = 8;
        let min_capacity_x = 1;
        let min_capacity_y = 1;
        let expected = vec![0; TILE_SIZE * TILE_SIZE];
        let result: TiledArray2D<_, TILE_SIZE> = super::TiledArray2D::with_min_capacity(min_capacity_x, min_capacity_y, 0);
        
        assert_eq!(result.as_slice(), expected);
    }

    #[test]
    fn test_one_tile_array_indexing() {
        const TILE_SIZE: usize = 8;
        let min_capacity_x = 1;
        let min_capacity_y = 1;
        let mut result: TiledArray2D<_, TILE_SIZE> = super::TiledArray2D::with_min_capacity(min_capacity_x, min_capacity_y, (0, 0));
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
    fn test_multiple_tile_array_storage() {
        const TILE_SIZE: usize = 8;
        let min_capacity_x = 16;
        let min_capacity_y = 24;
        let expected = vec![0; min_capacity_x * min_capacity_y];
        let result: TiledArray2D<_, TILE_SIZE> = super::TiledArray2D::with_min_capacity(min_capacity_x, min_capacity_y, 0);
        
        assert_eq!(result.as_slice(), expected);
    }

    #[test]
    fn test_multiple_tile_array_indexing() {
        const TILE_SIZE: usize = 8;
        let min_capacity_x = 16;
        let min_capacity_y = 24;
        let mut result: TiledArray2D<_, TILE_SIZE> = super::TiledArray2D::with_min_capacity(min_capacity_x, min_capacity_y, (0, 0));
        for row in 0..min_capacity_y {
            for col in 0..min_capacity_x {
                result[(row, col)] = (row, col);
            }
        }
        
        
    }

    #[test]
    fn test_tile_indexing1() {
        const TILE_SIZE: usize = 4;
        let min_capacity_x = 12;
        let min_capacity_y = 8;
        let mut result: TiledArray2D<_, TILE_SIZE> = super::TiledArray2D::with_min_capacity(min_capacity_x, min_capacity_y, 0);
        let (width_tiles, height_tiles) = result.shape_tiles();
        for tile_x in 0..width_tiles {
            for tile_y in 0..height_tiles {
                let value = width_tiles * tile_y + tile_x;
                for i in 0..TILE_SIZE { 
                    for j in 0..TILE_SIZE {
                        result[TileIndex(tile_x, tile_y)][i][j] = value;
                    }
                }
            }
        }
        let expected = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
            4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
            5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
        ];

        assert_eq!(result.as_slice(), expected);
    }

    #[test]
    fn test_tile_indexing2() {
        const TILE_SIZE: usize = 4;
        let min_capacity_x = 12;
        let min_capacity_y = 8;
        let mut result: TiledArray2D<_, TILE_SIZE> = super::TiledArray2D::with_min_capacity(min_capacity_x, min_capacity_y, 0);
        let (width_elements, height_elements) = result.shape_elements();
        for row in 0..height_elements {
            for col in 0..width_elements {
                let value = row * width_elements + col;
                result[(row, col)] = value;
            }
        }
        let expected = vec![
            0,  1,  2,  3,  12, 13, 14, 15, 24, 25, 26, 27, 36, 37, 38, 39,
            4,  5,  6,  7,  16, 17, 18, 19, 28, 29, 30, 31, 40, 41, 42, 43,
            8,  9,  10, 11, 20, 21, 22, 23, 32, 33, 34, 35, 44, 45, 46, 47,
            48, 49, 50, 51, 60, 61, 62, 63, 72, 73, 74, 75, 84, 85, 86, 87,
            52, 53, 54, 55, 64, 65, 66, 67, 76, 77, 78, 79, 88, 89, 90, 91,
            56, 57, 58, 59, 68, 69, 70, 71, 80, 81, 82, 83, 92, 93, 94, 95
        ];

        assert_eq!(result.as_slice(), expected);
    }
}

