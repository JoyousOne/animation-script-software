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

pub struct Position {
    x: usize,
    y: usize,
}
