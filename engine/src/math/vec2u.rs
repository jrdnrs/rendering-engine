#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Vec2u {
    pub x: u32,
    pub y: u32,
}

impl Vec2u {
    pub fn new(x: u32, y: u32) -> Self {
        Vec2u { x, y }
    }

    pub fn from_slice(slice: &[u32]) -> Self {
        Vec2u {
            x: slice[0],
            y: slice[1],
        }
    }

    pub fn from_array(slice: [u32; 2]) -> Self {
        Vec2u {
            x: slice[0],
            y: slice[1],
        }
    }

    pub fn as_tuple(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}