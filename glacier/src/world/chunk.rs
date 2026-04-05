use crate::world::block::BlockState;

pub type ChunkBlockList = [BlockState; 16 * 16 * 256];

pub struct Chunk {
    blocks: ChunkBlockList,
}

impl Chunk {
    pub fn new(blocks: ChunkBlockList) -> Self {
        Self { blocks }
    }

    pub fn get_block_at(&self, x: usize, y: usize, z: usize) -> BlockState {
        self.blocks[coord_to_index(x, y, z)]
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, block: BlockState) {
        self.blocks[coord_to_index(x, y, z)] = block;
    }
}

const fn coord_to_index(x: usize, y: usize, z: usize) -> usize {
    y * 16 * 16 + z * 16 + x
}