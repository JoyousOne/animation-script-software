use std::collections::{HashMap, hash_map::Entry};

use crate::{
    animator::types::Point,
    common::types::{IPosition, Limit},
};

pub struct Polygon {
    /// Line is define by y -> (min_x, max_x)
    pub lateral_line: HashMap<isize, Limit>,
}

impl Polygon {
    pub fn new() -> Self {
        Polygon {
            lateral_line: HashMap::new(),
        }
    }

    pub fn update(&mut self, p: IPosition) {
        let IPosition { x, y } = p;

        let limit = match self.lateral_line.entry(y) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(Limit { min: x, max: x }),
        };

        limit.update(x);
    }
}

pub fn mid_point_line_drawing(p1: Point, p2: Point) -> Vec<IPosition> {
    let mut line = Vec::new();

    let (x1, x2) = (p1.x as isize, p2.x as isize);
    let (y1, y2) = (p1.y as isize, p2.y as isize);

    let (dx, dy) = ((x2 - x1).abs(), (y2 - y1).abs());

    if dy <= dx {
        let mut d = dy - (dx / 2);

        let (mut start, end) = if p1.x <= p2.x {
            (
                IPosition::new(p1.x as isize, p1.y as isize),
                IPosition::new(p2.x as isize, p2.y as isize),
            )
        } else {
            (
                IPosition::new(p2.x as isize, p2.y as isize),
                IPosition::new(p1.x as isize, p1.y as isize),
            )
        };

        let op = if start.y > end.y { -1 } else { 1 };
        // println!("x: {}, y: {}", start.x, start.y);
        line.push(IPosition::new(start.x as isize, start.y as isize));

        while start.x < end.x {
            start.x += 1;

            if d < 0 {
                // EAST
                d += dy;
            } else {
                // NORTH-EAST
                d += dy - dx;
                start.y += op;
            }

            // println!("x: {}, y: {}", start.x, start.y);
            line.push(IPosition::new(start.x as isize, start.y as isize));
        }
    } else if dx < dy {
        let mut d = dx - (dy / 2);

        let (mut start, end) = if p1.y <= p2.y {
            (
                IPosition::new(p1.x as isize, p1.y as isize),
                IPosition::new(p2.x as isize, p2.y as isize),
            )
        } else {
            (
                IPosition::new(p2.x as isize, p2.y as isize),
                IPosition::new(p1.x as isize, p1.y as isize),
            )
        };

        let op = if start.x > end.x { -1 } else { 1 };

        // println!("x: {}, y: {}", start.x, start.y);
        line.push(IPosition::new(start.x as isize, start.y as isize));

        while start.y < end.y {
            start.y += 1;

            if d < 0 {
                // EAST
                d += dx;
            } else {
                // NORTH-EAST
                d += dx - dy;
                start.x += op;
            }

            // println!("x: {}, y: {}", start.x, start.y);
            line.push(IPosition::new(start.x as isize, start.y as isize));
        }
    }

    line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mid_point_line_drawing_test() {
        // Vertival line
        let p1 = Point { x: 0., y: 0. };
        let p2 = Point { x: 0., y: 5. };

        let line = mid_point_line_drawing(p1, p2);

        assert_eq!(
            line,
            vec![
                IPosition::new(0, 0),
                IPosition::new(0, 1),
                IPosition::new(0, 2),
                IPosition::new(0, 3),
                IPosition::new(0, 4),
                IPosition::new(0, 5),
            ]
        );

        let p1 = Point { x: 0., y: 5. };
        let p2 = Point { x: 0., y: 0. };

        let line = mid_point_line_drawing(p1, p2);

        assert_eq!(
            line,
            vec![
                IPosition::new(0, 0),
                IPosition::new(0, 1),
                IPosition::new(0, 2),
                IPosition::new(0, 3),
                IPosition::new(0, 4),
                IPosition::new(0, 5),
            ]
        );

        // Horizontal line
        let p1 = Point { x: 0., y: 0. };
        let p2 = Point { x: 5., y: 0. };

        let line = mid_point_line_drawing(p1, p2);

        assert_eq!(
            line,
            vec![
                IPosition::new(0, 0),
                IPosition::new(1, 0),
                IPosition::new(2, 0),
                IPosition::new(3, 0),
                IPosition::new(4, 0),
                IPosition::new(5, 0),
            ]
        );

        let p1 = Point { x: 5., y: 0. };
        let p2 = Point { x: 0., y: 0. };

        let line = mid_point_line_drawing(p1, p2);

        assert_eq!(
            line,
            vec![
                IPosition::new(0, 0),
                IPosition::new(1, 0),
                IPosition::new(2, 0),
                IPosition::new(3, 0),
                IPosition::new(4, 0),
                IPosition::new(5, 0),
            ]
        );

        // Diagonal
        let p1 = Point { x: 3., y: 2. };
        let p2 = Point { x: 17., y: 5. };

        let line = mid_point_line_drawing(p1, p2);

        assert_eq!(
            line,
            vec![
                IPosition::new(3, 2),
                IPosition::new(4, 2),
                IPosition::new(5, 2),
                IPosition::new(6, 3),
                IPosition::new(7, 3),
                IPosition::new(8, 3),
                IPosition::new(9, 3),
                IPosition::new(10, 4),
                IPosition::new(11, 4),
                IPosition::new(12, 4),
                IPosition::new(13, 4),
                IPosition::new(14, 4),
                IPosition::new(15, 5),
                IPosition::new(16, 5),
                IPosition::new(17, 5),
            ]
        );
    }
}
