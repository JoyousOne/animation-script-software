use std::ops::{Add, Mul, Sub};

use super::types::{CoordinateValue, Direction, EasingFunction, Point, Rotation};

#[derive(Clone, Copy)]
pub enum Transition {
    Translate(TransitionDescriptor<Point>),
    ScaleTop(TransitionDescriptor<CoordinateValue>),
    ScaleBottom(TransitionDescriptor<CoordinateValue>),
    ScaleLeft(TransitionDescriptor<CoordinateValue>),
    ScaleRight(TransitionDescriptor<CoordinateValue>),
    Rotate(TransitionDescriptor<Rotation>),
}

#[derive(Clone, Copy)]
pub struct TransitionDescriptor<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<f64, Output = T> + Clone,
{
    pub end_value: T,
    pub start_frame: u32,
    pub end_frame: u32,
    pub play_count: u32,
    pub easing_function: EasingFunction,
    pub direction: Direction,
}

impl<T: Add<Output = T> + Sub<Output = T> + Mul<f64, Output = T> + Clone> TransitionDescriptor<T> {
    pub fn calculate_value_at_frame(self, start_value: &T, frame_count: u32) -> T {
        if frame_count <= self.start_frame {
            start_value.clone()
        } else if frame_count >= self.end_frame {
            self.end_value
        } else {
            let timing_progress = f64::from(frame_count - self.start_frame)
                / f64::from(self.end_frame - self.start_frame);

            let distance = (self.end_value - start_value.clone()) * timing_progress;
            start_value.clone() + distance
        }
    }
}

pub trait Translate {
    fn position(&self) -> &Point;

    fn translate(&mut self, p: &Point);

    fn apply_translate_transition(
        &mut self,
        descriptor: &TransitionDescriptor<Point>,
        frame_count: u32,
    ) {
        self.translate(&descriptor.calculate_value_at_frame(self.position(), frame_count));
    }
}

pub trait Scale {
    fn top(&self) -> CoordinateValue;

    fn bottom(&self) -> CoordinateValue;

    fn left(&self) -> CoordinateValue;

    fn right(&self) -> CoordinateValue;

    fn scale_top(&mut self, value: CoordinateValue);

    fn scale_bottom(&mut self, value: CoordinateValue);

    fn scale_left(&mut self, value: CoordinateValue);

    fn scale_right(&mut self, value: CoordinateValue);

    fn apply_scale_top_transition(
        &mut self,
        descriptor: &TransitionDescriptor<CoordinateValue>,
        frame_count: u32,
    ) {
        self.scale_top(descriptor.calculate_value_at_frame(&self.top(), frame_count));
    }

    fn apply_scale_bottom_transition(
        &mut self,
        descriptor: &TransitionDescriptor<CoordinateValue>,
        frame_count: u32,
    ) {
        self.scale_bottom(descriptor.calculate_value_at_frame(&self.bottom(), frame_count));
    }

    fn apply_scale_left_transition(
        &mut self,
        descriptor: &TransitionDescriptor<CoordinateValue>,
        frame_count: u32,
    ) {
        self.scale_left(descriptor.calculate_value_at_frame(&self.left(), frame_count));
    }

    fn apply_scale_right_transition(
        &mut self,
        descriptor: &TransitionDescriptor<CoordinateValue>,
        frame_count: u32,
    ) {
        self.scale_right(descriptor.calculate_value_at_frame(&self.right(), frame_count));
    }
}

pub trait Rotate {
    fn rotation(&self) -> &Rotation;

    fn rotate(&mut self, r: Rotation);

    fn apply_rotate_transition(
        &mut self,
        descriptor: &TransitionDescriptor<Rotation>,
        frame_count: u32,
    ) {
        self.rotate(descriptor.calculate_value_at_frame(&self.rotation(), frame_count));
    }
}
