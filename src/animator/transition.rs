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
pub struct TransitionDescriptor<T> {
    pub end_value: T,
    pub start_frame: u32,
    pub end_frame: u32,
    pub play_count: u32,
    pub easing_function: EasingFunction,
    pub direction: Direction,
}

pub trait Translate {
    fn position(&self) -> &Point;

    fn translate(&mut self, p: &Point);

    fn apply_translate_transition(
        &mut self,
        descriptor: &TransitionDescriptor<Point>,
        frame_count: u32,
    ) {
        if frame_count >= descriptor.start_frame {
            let timing_progress = f64::from(frame_count - descriptor.start_frame)
                / f64::from(descriptor.end_frame - descriptor.start_frame);

            if timing_progress >= 1.0 {
                self.translate(&descriptor.end_value);
            } else {
                let initial_position = self.position();
                let total_distance = descriptor.end_value - *initial_position;
                let distance = total_distance * timing_progress;
                let result_position = *initial_position + distance;
                self.translate(&result_position);
            }
        }
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
        todo!()
    }

    fn apply_scale_bottom_transition(
        &mut self,
        descriptor: &TransitionDescriptor<CoordinateValue>,
        frame_count: u32,
    ) {
        todo!()
    }

    fn apply_scale_left_transition(
        &mut self,
        descriptor: &TransitionDescriptor<CoordinateValue>,
        frame_count: u32,
    ) {
        todo!()
    }

    fn apply_scale_right_transition(
        &mut self,
        descriptor: &TransitionDescriptor<CoordinateValue>,
        frame_count: u32,
    ) {
        todo!()
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
        todo!()
    }
}
