use crate::world::block::{AIR, BEDROCK, DIRT, GRASS_BLOCK};
use crate::world::chunk::Chunk;
use crate::worldgen::generator::WorldGenerator;

pub struct FlatWorldGenerator;

impl WorldGenerator for FlatWorldGenerator {
    fn generate_chunk(&self, _chunk_x: i32, _chunk_z: i32) -> Chunk {
        let mut chunk: Chunk = Chunk::new([AIR; 16 * 16 * 256]);

        for x in 0..16 {
            for z in 0..16 {
                for y in 0..256 {
                    let block = if y == 0 { BEDROCK } else if y == 1 || y == 2 { DIRT } else if y == 3 { GRASS_BLOCK } else { AIR };
                    chunk.set(x, y, z, block);
                }
            }
        }

        chunk
    }
}