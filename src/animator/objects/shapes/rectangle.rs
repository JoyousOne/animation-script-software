use crate::{
    animator::{
        scene::{Draw, Frame},
        transition::{Rotate, Scale, Transition, Transitionable, Translate},
        types::{Color, CoordinateValue, Length, Point, Rotation},
    },
    common::{
        drawing::{Polygon, mid_point_line_drawing},
        types::Limit,
    },
};

const BLACK: Color = Color::RGB(0, 0, 0);

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
}

impl Rectangle {
    /// FLUENT CHAINED SYNTAX STARTS HERE
    pub fn new(top_left: Point, bottom_right: Point) -> Self {
        Rectangle {
            p1: top_left,
            p2: bottom_right,
            rotation: Rotation::Turn(0.),
            z_index: 0, // TODO check if needs to be automatically incremented when auto
            fill_color: BLACK,
            border_color: BLACK,
            border_size: Length::Pixel(0),
            outline_color: BLACK,
            outline_size: Length::Pixel(0),
        }
    }

    pub fn rotation(&mut self, rotation: Rotation) -> &mut Self {
        self.rotation = rotation;
        self
    }

    pub fn z_index(&mut self, z_index: i32) -> &mut Self {
        self.z_index = z_index;
        self
    }

    pub fn fill_color(&mut self, fill_color: Color) -> &mut Self {
        self.fill_color = fill_color;
        self
    }

    pub fn border_color(&mut self, border_color: Color) -> &mut Self {
        self.border_color = border_color;
        self
    }

    pub fn border_size(&mut self, border_size: Length) -> &mut Self {
        self.border_size = border_size;
        self
    }

    pub fn outline_color(&mut self, outline_color: Color) -> &mut Self {
        self.outline_color = outline_color;
        self
    }

    pub fn outline_size(&mut self, outline_size: Length) -> &mut Self {
        self.outline_size = outline_size;
        self
    }
    /// FLUENT CHAINED SYNTAX ENDS HERE

    pub fn center(&self) -> Point {
        Point {
            x: (self.p1.x + self.p2.x) / 2.0,
            y: (self.p1.y + self.p2.y) / 2.0,
        }
    }

    fn original_corners(&self) -> [Point; 4] {
        let (x1, x2) = (self.p1.x, self.p2.x);
        let (y1, y2) = (self.p1.y, self.p2.y);
        let corners: [Point; 4] = [
            self.p1,
            Point { x: x2, y: y1 },
            Point { x: x1, y: y2 },
            self.p2,
        ];

        corners
    }

    pub fn corners(&self) -> Vec<Point> {
        let corners = [
            self.p1,
            Point {
                x: self.p2.x,
                y: self.p1.y,
            },
            Point {
                x: self.p1.x,
                y: self.p2.y,
            },
            self.p2,
        ];

        let center = self.center();

        let rotated_corners = corners
            .iter()
            .map(|&corner| {
                let relative_x = corner.x - center.x;
                let relative_y = corner.y - center.y;

                let rotated_relative_x =
                    relative_x * self.rotation.cos() - relative_y * self.rotation.sin();
                let rotated_relative_y =
                    relative_x * self.rotation.sin() + relative_y * self.rotation.cos();

                Point {
                    x: rotated_relative_x + center.x,
                    y: rotated_relative_y + center.y,
                }
            })
            .collect();

        rotated_corners
    }
}

impl Transitionable for Rectangle {
    fn apply_transition(&mut self, transition: &Transition, frame_count: u32) {
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

    fn draw(&self, _frame_count: u32, frame: &mut Frame) {
        let corners = self.corners();
        let mut border = Vec::new();

        // Horizontal parallel line
        border.extend_from_slice(&mid_point_line_drawing(corners[0], corners[1]));
        border.extend_from_slice(&mid_point_line_drawing(corners[2], corners[3]));

        // Vertical parallel line
        border.extend_from_slice(&mid_point_line_drawing(corners[0], corners[2]));
        border.extend_from_slice(&mid_point_line_drawing(corners[1], corners[3]));

        let mut rect = Polygon::new();
        for p in border {
            rect.update(p);
        }

        // DEBUG
        // for (y, Limit { min, max }) in &rect.lateral_line {
        //     println!("y: {y}, {min}->{max}");
        // }

        let max_y = frame.ysize() as isize;
        let max_x = frame.xsize() as isize;
        for (y, Limit { min, max }) in rect.lateral_line {
            // Skip line if out of bound
            if y < 0 || y > max_y {
                continue;
            }

            // Adjust the limit of x
            let x_start = min.max(0);
            let x_end = max.min(max_x);

            // Add pixels
            for x in x_start..=x_end {
                frame.put_pixel(x as usize, y as usize, self.fill_color);
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
