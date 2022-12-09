use super::block::{Block};
use super::position::{ShortPosition};


#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct Chunk {
    pub blocks: [Block; 16 * 16 * 16]
}

impl Chunk {
    #[inline]
    pub fn new() -> Chunk {
        Chunk {
            blocks: [Block::new(0, 0); 16 * 16 * 16]
        }
    }

    pub fn look_up(&self, pos: ShortPosition) -> Block {

        let index = pos.as_u4s() as usize;
        self.blocks[index]
    }

    pub fn set(&mut self, pos: ShortPosition, block: &Block) {
        let index = pos.as_u4s() as usize;

        if index > 16 * 16 * 16 {
            panic!("Index out of bounds");
        }
        self.blocks[index] = *block;
    }

    pub unsafe fn unsafe_set(&mut self, pos: ShortPosition, block: Block) {
        let index = pos.as_u4s() as usize;

        self.blocks[index] = block;
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        for block in self.blocks.iter() {
            bytes.push(block.id as u8);
            bytes.push((block.id >> 8) as u8);
            bytes.push((block.id >> 16) as u8);
            bytes.push((block.id >> 24) as u8);
            bytes.push(block.data as u8);
            bytes.push((block.data >> 8) as u8);
        }
        bytes
    }

    pub fn from_bytes(bytes: [u8; 16 * 16 * 16 * 6]) -> Chunk {
        let mut blocks: [Block; 16 * 16 * 16] = [Block::new(0, 0); 16 * 16 * 16];
        for i in 0..16 * 16 * 16 {
            let id = (bytes[i * 6] as u32) | ((bytes[i * 6 + 1] as u32) << 8) | ((bytes[i * 6 + 2] as u32) << 16) | ((bytes[i * 6 + 3] as u32) << 24);
            let data = (bytes[i * 6 + 4] as u16) | ((bytes[i * 6 + 5] as u16) << 8);
            blocks[i] = Block::new(id, data);
        }
        Chunk {
            blocks: blocks
        }
    }
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Chunk")
    }
}
