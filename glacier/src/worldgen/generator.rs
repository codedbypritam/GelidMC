use crate::world::chunk::Chunk;

pub trait WorldGenerator {
    fn generate_chunk(&self, _chunk_x: i32, _chunk_z: i32) -> Chunk;
}