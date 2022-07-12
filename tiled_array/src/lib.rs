use std::ops;


type Tile<T, const TILE_SIZE: usize> = [[T; TILE_SIZE]; TILE_SIZE];

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
struct TileIndex(usize, usize);


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
        let tile_x = self.tiles_x[_index.0];
        let tile_y = self.tiles_y[_index.1];
        let tile_index = self.tile_width * tile_y.index + tile_x.index;

        &self.data[tile_index][tile_x.offset][tile_y.offset]
    }
}

impl<T, const TILE_SIZE: usize> ops::IndexMut<(usize, usize)> for TiledArray2D<T, TILE_SIZE>
where
    T: Copy
{   
    #[inline]
    fn index_mut(&mut self, _index: (usize, usize)) -> &mut Self::Output {
        let tile_x = self.tiles_x[_index.0];
        let tile_y = self.tiles_y[_index.1];
        let tile_index = self.tile_width * tile_y.index + tile_x.index;

        &mut self.data[tile_index][tile_x.offset][tile_y.offset]
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
        for i in 0..8 {
            for j in 0..8 {
                result[(i, j)] = (i, j);
            }
        }
        
        for i in 0..8 {
            for j in 0..8 {
                assert_eq!(result[(i, j)], (i, j));
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
        for i in 0..min_capacity_x {
            for j in 0..min_capacity_y {
                result[(i, j)] = (i, j);
            }
        }
        
        for i in 0..min_capacity_x {
            for j in 0..min_capacity_y {
                assert_eq!(result[(i, j)], (i, j));
            }
        }
    }
}

