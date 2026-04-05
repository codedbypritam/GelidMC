pub mod world;
pub mod worldgen;

#[cfg(test)]
mod tests {
    use image::{ImageBuffer, Rgb};
    use crate::world::block::{AIR, BEDROCK, DIRT, GRASS_BLOCK};
    use crate::worldgen::{FlatWorldGenerator, WorldGenerator};

    #[test]
    fn test_flat_world_generator() {
        let generator: FlatWorldGenerator = FlatWorldGenerator;

        let chunk = generator.generate_chunk(0, 0);

        let mut image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(16, 16);
        let y: usize = 4;

        for x in 0..16 {
            for z in 0..16 {
                let block = chunk.get_block_at(x, y, z);

                if block == AIR {
                    image.put_pixel(x as u32, z as u32, Rgb([255, 255, 255]));
                } else if block == BEDROCK {
                    image.put_pixel(x as u32, z as u32, Rgb([0, 0, 0]));
                } else if block == DIRT {
                    image.put_pixel(x as u32, z as u32, Rgb([150, 75, 0]));
                } else if block == GRASS_BLOCK {
                    image.put_pixel(x as u32, z as u32, Rgb([0, 255, 0]));
                }
            }
        }

        image.save("flat_world.png").expect("Failed to save image.");
    }
}