#[derive(Clone, Debug, PartialEq)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IPosition {
    pub x: isize,
    pub y: isize,
}

impl IPosition {
    pub fn new(x: isize, y: isize) -> Self {
        IPosition { x, y }
    }
}

pub struct Limit {
    pub min: isize,
    pub max: isize,
}

impl Limit {
    pub fn update(&mut self, new_value: isize) {
        if self.min > new_value {
            self.min = new_value;
        }

        if self.max < new_value {
            self.max = new_value;
        }
    }
}
