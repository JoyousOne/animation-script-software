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

    pub fn sin(&self) -> f64 {
        self.as_radian().value().sin()
    }

    pub fn cos(&self) -> f64 {
        self.as_radian().value().cos()
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

pub fn cubic_bezier(t: f64, p0: Point, p1: Point, p2: Point, p3: Point) -> Point {
    let x = (1.0 - t).powi(3) * p0.x
        + 3.0 * (1.0 - t).powi(2) * t * p1.x
        + 3.0 * (1.0 - t) * t.powi(2) * p2.x
        + t.powi(3) * p3.x;

    let y = (1.0 - t).powi(3) * p0.y
        + 3.0 * (1.0 - t).powi(2) * t * p1.y
        + 3.0 * (1.0 - t) * t.powi(2) * p2.y
        + t.powi(3) * p3.y;

    Point { x, y }
}

pub fn cubic_bezier_derivative(t: f64, p0: Point, p1: Point, p2: Point, p3: Point) -> Point {
    let x = -3.0 * (1.0 - t).powi(2) * p0.x + 3.0 * (1.0 - t).powi(2) * p1.x
        - 6.0 * (1.0 - t) * t * p1.x
        + 6.0 * (1.0 - t) * t * p2.x
        - 3.0 * t.powi(2) * p2.x
        + 3.0 * t.powi(2) * p3.x;

    let y = -3.0 * (1.0 - t).powi(2) * p0.y + 3.0 * (1.0 - t).powi(2) * p1.y
        - 6.0 * (1.0 - t) * t * p1.y
        + 6.0 * (1.0 - t) * t * p2.y
        - 3.0 * t.powi(2) * p2.y
        + 3.0 * t.powi(2) * p3.y;

    Point { x, y }
}

pub fn newton_root_finding<F, D>(
    function: F,
    derivative: D,
    target: f64,
    initial_x: f64,
    max_x: f64,
    min_x: f64,
    max_iter: usize,
    convergence_threshold: f64,
) -> f64
where
    F: Fn(f64) -> f64,
    D: Fn(f64) -> f64,
{
    let mut x = initial_x;

    for i in 0..max_iter {
        let y = function(x);
        let dy = derivative(x);

        let new_x = x - (y - target) / dy;

        if (new_x - x).abs() < convergence_threshold {
            println!("i = {i} with diff: {:?}", (new_x - x).abs());
            return new_x;
        }

        x = new_x.clamp(max_x, min_x);
    }

    x
}

#[derive(Debug, Clone, Copy)]
pub enum EasingFunction {
    Linear,
    // Steps(Box<Self>), // TODO setup with jump end etc
    CubicBezier(f64, f64, f64, f64),
}

impl EasingFunction {
    pub fn apply(&self, x: f64) -> f64 {
        let x = x.clamp(0.0, 1.0);

        match self {
            Self::Linear => x,
            Self::CubicBezier(x1, y1, x2, y2) => {
                let p0 = Point { x: 0.0, y: 0.0 };
                let p1 = Point { x: *x1, y: *y1 };
                let p2 = Point { x: *x2, y: *y2 };
                let p3 = Point { x: 1.0, y: 1.0 };

                let t = newton_root_finding(
                    |t| cubic_bezier(t, p0, p1, p2, p3).x,
                    |t| cubic_bezier_derivative(t, p0, p1, p2, p3).x,
                    x,
                    0.5,
                    0.0,
                    1.0,
                    100,
                    1e-10,
                );

                let point = cubic_bezier(t, p0, p1, p2, p3);

                point.y
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
