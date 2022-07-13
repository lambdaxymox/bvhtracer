use std::ops;
use std::slice;


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
struct TileMap2D {
    tiles_x: Vec<TileEntry>,
    tiles_y: Vec<TileEntry>,
    width_tiles: usize,
    height_tiles: usize,
}

impl TileMap2D {
    fn with_min_capacity(min_capacity_x: usize, min_capacity_y: usize, tile_size: usize) -> Self {
        let tile_count_x = min_capacity_x / tile_size;
        let tile_count_y = min_capacity_y / tile_size;
        let remainder_x = min_capacity_x % tile_size;
        let remainder_y = min_capacity_y % tile_size;
        let padded_tile_count_x = if remainder_x != 0 { tile_count_x + 1 } else { tile_count_x };
        let padded_tile_count_y = if remainder_y != 0 { tile_count_y + 1 } else { tile_count_y };
        let tile_element_count = tile_size * tile_size;
        let padded_capacity = padded_tile_count_x * padded_tile_count_y;
        let padded_element_width = padded_tile_count_x * tile_size;
        let padded_element_height = padded_tile_count_y * tile_size;

        let mut tiles_x = vec![];
        for col in 0..padded_element_width {
            let tile_index = col / tile_size;
            let index = tile_index;
            let offset = col % tile_size;
            let entry = TileEntry::new(index, offset);
            
            tiles_x.push(entry);
        }

        let mut tiles_y = vec![];
        for row in 0..padded_element_height {
            let tile_index = row / tile_size;
            let index = padded_tile_count_x * tile_index;
            let offset = row % tile_size;
            let entry = TileEntry::new(index, offset);
            
            tiles_y.push(entry);
        }

        Self {
            tiles_x,
            tiles_y,
            width_tiles: padded_tile_count_x,
            height_tiles: padded_tile_count_y,
        }
    }

    #[inline]
    fn width_tiles(&self) -> usize {
        self.width_tiles
    }

    #[inline]
    fn height_tiles(&self) -> usize {
        self.height_tiles
    }

    #[inline]
    fn width(&self) -> usize {
        self.tiles_x.len()
    }

    #[inline]
    fn height(&self) -> usize {
        self.tiles_y.len()
    }

    #[inline]
    fn get_unchecked(&self, index: (usize, usize)) -> (usize, usize, usize) {
        let tile_x = self.tiles_x[index.1];
        let tile_y = self.tiles_y[index.0];
        // let tile_index = self.tile_width * tile_y.index + tile_x.index;
        let tile_index = tile_x.index + tile_y.index;

        (tile_index, tile_y.offset, tile_x.offset)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct TiledArray2D<T, const TILE_SIZE: usize> {
    tile_map: TileMap2D,
    data: Vec<Tile<T, TILE_SIZE>>,
}

impl<T, const TILE_SIZE: usize> TiledArray2D<T, TILE_SIZE> 
where
    T: Copy,
{
    pub fn with_min_capacity(min_capacity_x: usize, min_capacity_y: usize, default_value: T) -> Self {
        let tile_map = TileMap2D::with_min_capacity(min_capacity_x, min_capacity_y, TILE_SIZE);
        let default_array = [[default_value; TILE_SIZE]; TILE_SIZE];
        let data = vec![default_array; tile_map.width_tiles() * tile_map.height_tiles()];

        Self { tile_map, data }
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
        // (self.tile_width * TILE_SIZE, self.tile_height * TILE_SIZE)
        (self.tile_map.width(), self.tile_map.height())
    }

    #[inline]
    pub fn shape_tiles(&self) -> (usize, usize) {
        // (self.tile_width, self.tile_height)
        (self.tile_map.width_tiles(), self.tile_map.height_tiles())
    }

    #[inline]
    pub fn width_tiles(&self) -> usize {
        // self.tile_width * TILE_SIZE
        self.tile_map.width_tiles()
    }

    #[inline]
    pub fn height_tiles(&self) -> usize {
        // self.tile_height * TILE_SIZE
        self.tile_map.height_tiles()
    }

    #[inline]
    pub fn width_elements(&self) -> usize {
        // self.tile_width * TILE_SIZE
        self.tile_map.width()
    }

    #[inline]
    pub fn height_elements(&self) -> usize {
        // self.tile_height * TILE_SIZE
        self.tile_map.height()
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

pub struct TileIter<'a, T, const TILE_SIZE: usize> {
    iter: slice::Iter<'a, Tile<T, TILE_SIZE>>,
}

impl<'a, T, const TILE_SIZE: usize> Iterator for TileIter<'a, T, TILE_SIZE> {
    type Item = &'a Tile<T, TILE_SIZE>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, T, const TILE_SIZE: usize> DoubleEndedIterator for TileIter<'a, T, TILE_SIZE> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<T, const TILE_SIZE: usize> TiledArray2D<T, TILE_SIZE> {
    pub fn tile_iter(&self) -> TileIter<T, TILE_SIZE> {
        TileIter {
            iter: self.data.iter(),
        }
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
        let (tile_index, offset_row, offset_col) = self.tile_map.get_unchecked(_index);

        &self.data[tile_index][offset_row][offset_col]
    }
}

impl<T, const TILE_SIZE: usize> ops::IndexMut<(usize, usize)> for TiledArray2D<T, TILE_SIZE>
where
    T: Copy
{   
    #[inline]
    fn index_mut(&mut self, _index: (usize, usize)) -> &mut Self::Output {
        let (tile_index, offset_row, offset_col) = self.tile_map.get_unchecked(_index);

        &mut self.data[tile_index][offset_row][offset_col]
    }
}

impl<T, const TILE_SIZE: usize> ops::Index<TileIndex> for TiledArray2D<T, TILE_SIZE>
where
    T: Copy
{
    type Output = Tile<T, TILE_SIZE>;
    
    #[inline]
    fn index(&self, _index: TileIndex) -> &Self::Output {
        let tile_index = self.width_tiles() * _index.1 + _index.0;

        &self.data[tile_index]
    }
}

impl<T, const TILE_SIZE: usize> ops::IndexMut<TileIndex> for TiledArray2D<T, TILE_SIZE>
where
    T: Copy
{   
    #[inline]
    fn index_mut(&mut self, _index: TileIndex) -> &mut Self::Output {
        let tile_index = self.width_tiles() * _index.1 + _index.0;

        &mut self.data[tile_index]
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_one_tile_array_storage() {
        const TILE_SIZE: usize = 8;
        let min_capacity_x = 1;
        let min_capacity_y = 1;
        let expected = vec![0; TILE_SIZE * TILE_SIZE];
        let result: TiledArray2D<_, TILE_SIZE> = TiledArray2D::with_min_capacity(min_capacity_x, min_capacity_y, 0);
        
        assert_eq!(result.as_slice(), expected);
    }

    #[test]
    fn test_one_tile_array_indexing() {
        const TILE_SIZE: usize = 8;
        let min_capacity_x = 1;
        let min_capacity_y = 1;
        let mut result: TiledArray2D<_, TILE_SIZE> = TiledArray2D::with_min_capacity(min_capacity_x, min_capacity_y, (0, 0));
        let (width_elements, height_elements) = result.shape_elements();
        for row in 0..height_elements {
            for col in 0..width_elements {
                result[(row, col)] = (row, col);
            }
        }
        
        let expected = vec![
            (0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7),
            (1, 0), (1, 1), (1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (1, 7),
            (2, 0), (2, 1), (2, 2), (2, 3), (2, 4), (2, 5), (2, 6), (2, 7),
            (3, 0), (3, 1), (3, 2), (3, 3), (3, 4), (3, 5), (3, 6), (3, 7),
            (4, 0), (4, 1), (4, 2), (4, 3), (4, 4), (4, 5), (4, 6), (4, 7),
            (5, 0), (5, 1), (5, 2), (5, 3), (5, 4), (5, 5), (5, 6), (5, 7),
            (6, 0), (6, 1), (6, 2), (6, 3), (6, 4), (6, 5), (6, 6), (6, 7),
            (7, 0), (7, 1), (7, 2), (7, 3), (7, 4), (7, 5), (7, 6), (7, 7),
        ];

        assert_eq!(result.as_slice(), expected);
    }

    #[test]
    fn test_multiple_tile_array_storage() {
        const TILE_SIZE: usize = 8;
        let min_capacity_x = 16;
        let min_capacity_y = 24;
        let expected = vec![0; min_capacity_x * min_capacity_y];
        let result: TiledArray2D<_, TILE_SIZE> = TiledArray2D::with_min_capacity(min_capacity_x, min_capacity_y, 0);
        
        assert_eq!(result.as_slice(), expected);
    }

    #[test]
    fn test_multiple_tile_array_indexing() {
        const TILE_SIZE: usize = 4;
        let min_capacity_x = 12;
        let min_capacity_y = 8;
        let mut result: TiledArray2D<_, TILE_SIZE> = TiledArray2D::with_min_capacity(min_capacity_x, min_capacity_y, (0, 0));
        let (width_elements, height_elements) = result.shape_elements();
        for row in 0..height_elements {
            for col in 0..width_elements {
                result[(row, col)] = (row, col);
            }
        }
        
        let expected = vec![
            (0, 0), (0, 1), (0, 2),  (0, 3),  (1, 0), (1, 1), (1, 2),  (1, 3),  (2, 0), (2, 1), (2, 2),  (2, 3),  (3, 0), (3, 1), (3, 2),  (3, 3),
            (0, 4), (0, 5), (0, 6),  (0, 7),  (1, 4), (1, 5), (1, 6),  (1, 7),  (2, 4), (2, 5), (2, 6),  (2, 7),  (3, 4), (3, 5), (3, 6),  (3, 7),
            (0, 8), (0, 9), (0, 10), (0, 11), (1, 8), (1, 9), (1, 10), (1, 11), (2, 8), (2, 9), (2, 10), (2, 11), (3, 8), (3, 9), (3, 10), (3, 11),
            (4, 0), (4, 1), (4, 2),  (4, 3),  (5, 0), (5, 1), (5, 2),  (5, 3),  (6, 0), (6, 1), (6, 2),  (6, 3),  (7, 0), (7, 1), (7, 2),  (7, 3),
            (4, 4), (4, 5), (4, 6),  (4, 7),  (5, 4), (5, 5), (5, 6),  (5, 7),  (6, 4), (6, 5), (6, 6),  (6, 7),  (7, 4), (7, 5), (7, 6),  (7, 7), 
            (4, 8), (4, 9), (4, 10), (4, 11), (5, 8), (5, 9), (5, 10), (5, 11), (6, 8), (6, 9), (6, 10), (6, 11), (7, 8), (7, 9), (7, 10), (7, 11)
        ];

        assert_eq!(result.as_slice(), expected);
    }

    #[test]
    fn test_tile_indexing1() {
        const TILE_SIZE: usize = 4;
        let min_capacity_x = 12;
        let min_capacity_y = 8;
        let mut result: TiledArray2D<_, TILE_SIZE> = TiledArray2D::with_min_capacity(min_capacity_x, min_capacity_y, 0);
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
        let mut result: TiledArray2D<_, TILE_SIZE> = TiledArray2D::with_min_capacity(min_capacity_x, min_capacity_y, 0);
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

