use crate::animator::{
    scene::{Draw, Frame},
    transition::{Rotate, Scale, Transition, Translate},
    types::{Color, CoordinateValue, Length, Point, Rotation},
};

#[derive(Clone)]
pub struct Rectangle {
    pub p1: Point,
    pub p2: Point,
    pub rotation: Rotation,
    pub z_index: i32,
    pub fill_color: Color,
    pub border_color: Color,
    pub border_size: Length,
    pub outline_color: Color,
    pub outline_size: Length,
    pub transitions: Vec<Transition>,
}

impl Rectangle {
    // TODO transitions should not be contained in the shape but as a super object
    pub fn apply_transitions(&mut self, frame_count: u32) {
        let t: Vec<Transition> = self.transitions.to_vec();

        for transition in t {
            self.apply_transition(&transition, frame_count);
        }
    }

    pub fn apply_transition(&mut self, transition: &Transition, frame_count: u32) {
        match transition {
            Transition::Translate(descriptor) => {
                self.apply_translate_transition(descriptor, frame_count)
            }
            Transition::ScaleTop(descriptor) => {
                self.apply_scale_top_transition(descriptor, frame_count);
            }
            Transition::ScaleBottom(descriptor) => {
                self.apply_scale_bottom_transition(descriptor, frame_count);
            }
            Transition::ScaleLeft(descriptor) => {
                self.apply_scale_left_transition(descriptor, frame_count);
            }
            Transition::ScaleRight(descriptor) => {
                self.apply_scale_right_transition(descriptor, frame_count);
            }
            Transition::Rotate(descriptor) => {
                self.apply_rotate_transition(descriptor, frame_count);
            }
        }
    }
}

impl Draw for Rectangle {
    fn zindex(&self) -> i32 {
        self.z_index
    }

    fn draw(&self, frame_count: u32, frame: &mut Frame) {
        let mut object = self.clone();

        object.apply_transitions(frame_count);

        let min_x = object.left().max(0.0) as usize;
        let max_x = object.right().min(frame.xsize() as f64) as usize;
        let min_y = object.top().max(0.0) as usize;
        let max_y = object.bottom().min(frame.ysize() as f64) as usize;

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                frame.put_pixel(x, y, object.fill_color);
            }
        }
    }
}

impl Translate for Rectangle {
    fn position(&self) -> &Point {
        &self.p1
    }

    fn translate(&mut self, p: &Point) {
        let start_x = self.p1.x;
        let start_y = self.p1.y;
        let end_x = p.x;
        let end_y = p.y;

        let dist_x = end_x - start_x;
        let dist_y = end_y - start_y;

        self.p1 = Point {
            x: self.p1.x + dist_x,
            y: self.p1.y + dist_y,
        };
        self.p2 = Point {
            x: self.p2.x + dist_x,
            y: self.p2.y + dist_y,
        };
    }
}

impl Scale for Rectangle {
    fn top(&self) -> CoordinateValue {
        f64::min(self.p1.y, self.p2.y)
    }

    fn bottom(&self) -> CoordinateValue {
        f64::max(self.p1.y, self.p2.y)
    }

    fn left(&self) -> CoordinateValue {
        f64::min(self.p1.x, self.p2.x)
    }

    fn right(&self) -> CoordinateValue {
        f64::max(self.p1.x, self.p2.x)
    }

    fn scale_top(&mut self, value: CoordinateValue) {
        if self.p1.y < self.p2.y {
            self.p1.y = value;
        } else {
            self.p2.y = value;
        }
    }

    fn scale_bottom(&mut self, value: CoordinateValue) {
        if self.p1.y > self.p2.y {
            self.p1.y = value;
        } else {
            self.p2.y = value;
        }
    }

    fn scale_left(&mut self, value: CoordinateValue) {
        if self.p1.x < self.p2.x {
            self.p1.x = value;
        } else {
            self.p2.x = value;
        }
    }

    fn scale_right(&mut self, value: CoordinateValue) {
        if self.p1.x > self.p2.x {
            self.p1.x = value;
        } else {
            self.p2.x = value;
        }
    }
}

impl Rotate for Rectangle {
    fn rotation(&self) -> &Rotation {
        &self.rotation
    }

    fn rotate(&mut self, r: Rotation) {
        self.rotation = r;
    }
}
