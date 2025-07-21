#[derive(Clone, Debug, PartialEq)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Pixel { r, g, b }
    }
}

pub type Color = Pixel;

impl Color {
    pub const fn as_bytes(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

#[derive(Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }
}
