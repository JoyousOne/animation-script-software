use std::{
    f64::consts::PI,
    ops::{Add, Mul, Sub},
};

pub type CoordinateValue = f64;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: CoordinateValue,
    pub y: CoordinateValue,
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

impl Mul<CoordinateValue> for Point {
    type Output = Self;

    fn mul(self, scalar: CoordinateValue) -> Self::Output {
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
    Percentage(f64),
}

impl Add for Length {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        todo!()
    }
}

impl Sub for Length {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        todo!()
    }
}

impl Mul<f64> for Length {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Rotation {
    Turn(f64),
    Degree(f64),
    Radian(f64),
}

impl Rotation {
    fn value(&self) -> f64 {
        match self {
            Self::Turn(value) => *value,
            Self::Degree(value) => *value,
            Self::Radian(value) => *value,
        }
    }

    pub fn as_degree(self) -> Self {
        match self {
            Self::Turn(value) => Self::Degree(value * 360.0),
            Self::Degree(_) => self,
            Self::Radian(value) => Self::Degree(value * (180.0 / PI)),
        }
    }

    pub fn as_turn(self) -> Self {
        match self {
            Self::Turn(_) => self,
            Self::Degree(value) => Self::Turn(value / 360.0),
            Self::Radian(value) => Self::Turn(value / (2.0 * PI)),
        }
    }

    pub fn as_radian(self) -> Self {
        match self {
            Self::Turn(value) => Self::Radian(value * (2.0 * PI)),
            Self::Degree(value) => Self::Radian(value * (PI / 180.0)),
            Self::Radian(_) => self,
        }
    }
}

impl Add for Rotation {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match self {
            Self::Turn(value) => Self::Turn(value + other.as_turn().value()),
            Self::Degree(value) => Self::Degree(value + other.as_degree().value()),
            Self::Radian(value) => Self::Radian(value + other.as_radian().value()),
        }
    }
}

impl Sub for Rotation {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match self {
            Self::Turn(value) => Self::Turn(value - other.as_turn().value()),
            Self::Degree(value) => Self::Degree(value - other.as_degree().value()),
            Self::Radian(value) => Self::Radian(value - other.as_radian().value()),
        }
    }
}

impl Mul<f64> for Rotation {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        match self {
            Self::Turn(value) => Self::Turn(value * scalar),
            Self::Degree(value) => Self::Degree(value * scalar),
            Self::Radian(value) => Self::Radian(value * scalar),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EasingFunction {
    Linear,
    // Steps(Box<Self>), // TODO setup with jump end etc
    CubicBezier(f64, f64, f64, f64),
}

impl EasingFunction {
    pub fn apply(&self, x: f64) -> f64 {
        let x = if x < 0.0 {
            0.0
        } else if x > 1.0 {
            1.0
        } else {
            x
        };

        match self {
            Self::Linear => x,
            Self::CubicBezier(p0, p1, p2, p3) => {
                (1.0 - x).powi(3) * p0
                    + 3.0 * (1.0 - x).powi(2) * x * p1
                    + 3.0 * (1.0 - x) * x.powi(2) * p2
                    + x.powi(3) * p3
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}
