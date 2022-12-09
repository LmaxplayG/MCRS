use super::block::{Block};
use super::position::{LongPosition,ShortPosition};
use super::chunk::{Chunk};

use std::collections::HashMap;

pub static WORLD_HEIGHT: i64 = 24;

pub struct World {
    pub chunks: HashMap<LongPosition, Chunk>
}

impl World {
    pub fn new() -> World {
        World {
            chunks: HashMap::new()
        }
    }

    pub fn look_up(&self, pos: LongPosition) -> &Chunk {
        let chunk_pos = LongPosition::new(pos.pos.0 / 16, pos.pos.1 / 16, pos.pos.2 / 16);
        let chunk = self.chunks.get(&chunk_pos).unwrap();
        chunk
    }

    pub fn set_chunk(&mut self, pos: LongPosition, chunk: &Chunk) {
        self.chunks.insert(pos, chunk.clone());
    }

    pub fn set(&mut self, pos: LongPosition, block: &Block) {
        // Split first 4 bits into block_pos
        let block_pos = ShortPosition::new((pos.pos.0 & 0xF) as u8, (pos.pos.1 & 0xF) as u8, (pos.pos.2 & 0xF) as u8);
        let chunk_pos = LongPosition::new(pos.pos.0 / 16, pos.pos.1 / 16, pos.pos.2 / 16);
        let chunk = self.chunks.entry(chunk_pos).or_insert(Chunk::new());
        
        unsafe {
            chunk.unsafe_set(block_pos, *block);
        }
    }

    pub fn read(&self, pos: LongPosition) -> Block {
        let chunk = self.look_up(pos);
        let block_pos = ShortPosition::new((pos.pos.0 % 16) as u8, (pos.pos.1 % 16) as u8, (pos.pos.2 % 16) as u8);
        chunk.look_up(block_pos)
    }

    pub fn add_all(&mut self, other: HashMap<LongPosition, Chunk>) {
        self.chunks.extend(other);
    }
}