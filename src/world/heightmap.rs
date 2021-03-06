use crate::render::constants::*;
use crate::util;

pub struct Heightmap {
    pub(super) data: Vec<isize>,
}

impl Heightmap {
    pub fn new() -> Heightmap {
        Heightmap {
            data: vec![0; CHUNK_SIZE * CHUNK_SIZE],
        }
    }

    pub fn get(&self, coord: &(usize, usize)) -> isize {
        self.data[util::coord_to_index_2d(&coord, CHUNK_SIZE)]
    }
}
