#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShortPosition {
    pub pos: (u8, u8, u8),
}

impl ShortPosition {
    #[inline]
    pub fn new(x: u8, y: u8, z: u8) -> ShortPosition {
        ShortPosition {
            pos: (x, y, z)
        }
    }

    pub fn as_u4s(&self) -> u16 {
        // Converts the u8s that make this up into a u16
        // 0xXXYYZZ -> 0xXYZ0
        ((self.pos.0 as u16) << 8) | ((self.pos.1 as u16) << 4) | (self.pos.2 as u16)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LongPosition {
    pub pos: (i64, i64, i64)
}

impl LongPosition {
    #[inline]
    pub fn new(x: i64, y: i64, z: i64) -> LongPosition {
        LongPosition {
            pos: (x, y, z)
        }
    }
}
