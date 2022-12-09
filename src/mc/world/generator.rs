// Uses simplex noise to generate a world
use super::world::{World, WORLD_HEIGHT};
use super::position::{LongPosition, ShortPosition};
use super::chunk::Chunk;
use super::block::{Block, Blocks};

use noise::{NoiseFn, Perlin};

pub fn generate_world(world: &mut World, seed: u32, pos: LongPosition) {
    let seed = seed;
    let perlin = Perlin::new(seed);

    let mut chunk = Chunk::new();

    for x in 0..16 {
        for z in 0..16 {
            let height = (perlin.get([((pos.pos.0 + x as i64) as f64) / 16.0, ((pos.pos.2 + z as i64) as f64) / 16.0]) * 8.0) as i64 + 8;
            for y in 0..WORLD_HEIGHT {
                let block_pos = ShortPosition::new(x, y as u8, z);
                unsafe {
                    if y < height {
                        chunk.unsafe_set(block_pos, Block { id: Blocks::STONE as u32, data: 0 });
                    } else if y == height {
                        chunk.unsafe_set(block_pos, Block { id: Blocks::GRASS as u32, data: 0 });
                    } else if y < height + 4 {
                        chunk.unsafe_set(block_pos, Block { id: Blocks::DIRT as u32, data: 0 });
                    } else {
                        chunk.unsafe_set(block_pos, Block { id: Blocks::AIR as u32, data: 0 });
                    }
                }
            }
        }
    }
    // println!("Generated chunk at {}, {}, {}", pos.pos.0, pos.pos.1, pos.pos.2);
    world.set_chunk(pos, &chunk);
}