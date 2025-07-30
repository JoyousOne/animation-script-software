use std::ops::{Add, Mul, Sub};

pub type CoordinateValue = f64;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: CoordinateValue,
    pub y: CoordinateValue,
}

impl Point {
    pub fn dist_x(p1: &Point, p2: &Point) -> CoordinateValue {
        (p1.x - p2.x).abs()
    }

    pub fn dist_y(p1: &Point, p2: &Point) -> CoordinateValue {
        (p1.y - p2.y).abs()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8),
    HEX(u32),
    HSL(u32, u32, u32),
    HSLA(u32, u32, u32, u32),
}

#[derive(Debug, Clone, Copy)]
pub enum Length {
    Pixel(u32),
    Percentage(u32),
    ScreenHeight(),
}

#[derive(Debug, Clone, Copy)]
pub enum Rotation {
    Degree(f64),
    Turn(f64),
    Radian(f64),
}

#[derive(Debug, Clone, Copy)]
pub enum EasingFunction {
    Linear,
    // Steps(Box<Self>), // TODO setup with jump end etc
    CubicBezier(f64, f64, f64, f64),
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

#[derive(Debug, Clone, Copy)]
pub enum Time {
    Frames(u32),
    Seconds(f64),
}

impl Time {
    fn as_frame_count(&self, fps: u32) -> u32 {
        match self {
            Self::Frames(count) => *count,
            Self::Seconds(count) => (*count * f64::from(fps)) as u32,
        }
    }
}
