#![allow(dead_code, unused_imports)]
mod mc;

use std::{io::Write, str::Bytes, collections::HashMap};

use mc::world::{Chunk, Block, ShortPosition, LongPosition, World, WORLD_HEIGHT, generate_world};

use rayon::prelude::*;

fn main() {

    let mut world = World::new();

    // Measure time
    let start = std::time::Instant::now();

    // generate a 100 chunk radius if not exists
    let radius = 100;

    for x in -radius..radius {
        for z in -radius..radius {
            for y in -8..16 {
                let chunk_pos = LongPosition::new(x, y, z);
                generate_world(&mut world, 0, chunk_pos);
            }
            // println!("Generated chunk at {}, {}", x, z);
        }
        println!("Generated chunk at {}", x);
    }

    // Save chunk as a .bin
    // Go through the chunks, and append them to chunks.bin
    let mut file = std::fs::File::create("chunks.bin").unwrap();

    // Write the chunks to the file
    for chunk in world.chunks.values() {
        file.write_all(&chunk.to_bytes()).unwrap();
        file.flush().unwrap();
    }

    println!("Saved chunks.bin");

    // Print time since start
    println!("Time elapsed: {}ms", start.elapsed().as_millis());
}
