// Make it as packed as possible
#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct Block {
    pub id: u32,
    pub data: u16
}

impl Block {
    #[inline]
    pub fn new(id: u32, data: u16) -> Block {
        Block {
            id: id,
            data: data
        }
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let string: String = format!("Block {{ id: {}, data: {} }}", self.id as u32, self.data as u16);
        write!(f, "{}", string)
    }
}

#[repr(u32)] // u32 is used for block ids
pub enum Blocks {
    AIR,
    STONE,
    GRASS,
    DIRT,
}