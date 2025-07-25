use std::fmt::{Debug, Display};

use crate::common::types::Position;

#[derive(Clone, Debug, PartialEq)]
pub struct RLE<T> {
    pub freq: usize,
    pub value: T,
}

impl<T> RLE<T> {
    pub fn new(freq: usize, value: T) -> Self {
        RLE { freq, value }
    }
}

pub trait ImageFormat<T> {
    /// Fill the image with a given color
    fn fill(&mut self, color: &T) -> &mut Self;

    /// Draw a rectangle from a position that start from the top left and that goes to a position bottom right with a given color.
    fn draw_rectangle(
        &mut self,
        top_left: Position,
        bottom_right: Position,
        color: &T,
    ) -> &mut Self;
}

pub struct Image<T> {
    height: usize,
    width: usize,
    pub pixels_indexes: Vec<RLE<T>>,
}

impl<T> Image<T>
where
    T: Clone + Copy + Display + Debug,
{
    pub fn new(height: usize, width: usize, default_color: T) -> Self {
        Image {
            height,
            width,
            pixels_indexes: vec![RLE::new((width * height) as usize, default_color.clone())],
        }
    }

    fn add_interval(&mut self, interval_start: usize, interval_end: usize, color: T) {
        let freq = interval_end - interval_start + 1;

        // Indexes of when an interval start or end is found
        let (mut index_start, mut index_end) = (0, 0);
        // current interval bounds (eg. [1, 4]);
        let (mut start_count, mut end_count) = (0, 0);

        let mut current_index = 0;
        let mut new_entries = vec![RLE::new(freq, color)];
        for p in &mut self.pixels_indexes {
            end_count += p.freq;

            // If start of interval in current interval
            if start_count <= interval_start && interval_start < end_count {
                let diff = interval_start - start_count;
                if diff > 0 {
                    new_entries.insert(0, RLE::new(diff as usize, p.value));
                }
                index_start = current_index;
            }

            // If end of interval in current interval
            if start_count < interval_end && interval_end <= end_count {
                index_end = current_index;

                let diff = end_count as isize - (interval_end + 1) as isize;
                if diff > 0 {
                    // Adding entry after the currently added value
                    new_entries.push(RLE::new(diff as usize, p.value));
                } else if diff < 0 {
                    // Reducing the size of the next entry
                    self.pixels_indexes[index_end + 1].freq -= diff.abs() as usize;
                }

                // Since the index_start must be set if we reach index_end
                break;
            }

            start_count = end_count;
            current_index += 1;
        }
        // DEBUG
        // println!("new_entries: {new_entries:?}");

        // Adding the new entries and replacing the one changed
        self.pixels_indexes
            .splice(index_start..index_end + 1, new_entries);
    }

    pub fn fill(&mut self, color: T) -> &mut Self {
        let freq = self.height * self.width;
        self.pixels_indexes = vec![RLE::new(freq, color)];
        self
    }

    pub fn draw_rectangle(&mut self, start: Position, end: Position, color: T) -> &mut Self {
        for y in start.y..end.y + 1 {
            let interval_start = y * self.width as usize + start.x;
            let interval_end = y * self.width as usize + end.x;
            self.add_interval(interval_start, interval_end, color);
        }

        self
    }

    pub fn print_content(&self) {
        let mut i = 1;
        for RLE { freq, value } in &self.pixels_indexes {
            for _ in 1..*freq + 1 {
                print!("{value} ");
                if i % self.width == 0 {
                    println!();
                }
                i += 1;
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_add_interval_within_same_index_test() {
        // [(4, 'A')] ==> [(2, 'B'), (2, 'A')]
        let mut img = Image::new(1, 4, 'A');
        img.add_interval(0, 1, 'B');
        assert_eq!(img.pixels_indexes, vec![RLE::new(2, 'B'), RLE::new(2, 'A')]);

        // [(4, 'A')] ==> [(2, 'A'), (2, 'B')]
        let mut img = Image::new(1, 4, 'A');
        img.add_interval(2, 3, 'B');
        assert_eq!(img.pixels_indexes, vec![RLE::new(2, 'A'), RLE::new(2, 'B')]);

        // [(4, 'A')] ==> [(1, 'A'), (2, 'B'), (1, 'A')]
        let mut img = Image::new(1, 4, 'A');
        img.add_interval(1, 2, 'B');
        assert_eq!(
            img.pixels_indexes,
            vec![RLE::new(1, 'A'), RLE::new(2, 'B'), RLE::new(1, 'A')]
        );

        // [(4, 'A')] ==> [(4, 'B')]
        let mut img = Image::new(1, 4, 'A');
        img.add_interval(0, 3, 'B');
        assert_eq!(img.pixels_indexes, vec![RLE::new(4, 'B')]);
    }

    #[test]
    fn image_add_interval_with_different_index_test() {
        // [(2, 'A'), (2, 'C')] ==> [(2, 'B'), (2, 'C')]
        let mut img = Image::new(1, 4, 'A');
        img.pixels_indexes = vec![RLE::new(2, 'A'), RLE::new(2, 'C')];
        img.add_interval(0, 1, 'B');
        assert_eq!(img.pixels_indexes, vec![RLE::new(2, 'B'), RLE::new(2, 'C')]);

        // [(2, 'A'), (2, 'C')] ==> [(2, 'A'), (2, 'B')]
        let mut img = Image::new(1, 4, 'A');
        img.pixels_indexes = vec![RLE::new(2, 'A'), RLE::new(2, 'C')];
        img.add_interval(2, 3, 'B');
        assert_eq!(img.pixels_indexes, vec![RLE::new(2, 'A'), RLE::new(2, 'B')]);

        // [(2, 'A'), (2, 'C')] ==> [(1, 'A'), (2, 'B'), (1, 'C')]
        let mut img = Image::new(1, 4, 'A');
        img.pixels_indexes = vec![RLE::new(2, 'A'), RLE::new(2, 'C')];
        img.add_interval(1, 2, 'B');
        assert_eq!(
            img.pixels_indexes,
            vec![RLE::new(1, 'A'), RLE::new(2, 'B'), RLE::new(1, 'C')]
        );

        // [(2, 'A'), (3, 'C')] ==> [(1, 'A'), (2, 'B'), (2, 'C')]
        let mut img = Image::new(1, 5, 'A');
        img.pixels_indexes = vec![RLE::new(2, 'A'), RLE::new(3, 'C')];
        img.add_interval(1, 2, 'B');
        assert_eq!(
            img.pixels_indexes,
            vec![RLE::new(1, 'A'), RLE::new(2, 'B'), RLE::new(2, 'C')]
        );

        // [(2, 'A'), (2, 'C')] ==> [(4, 'B')]
        let mut img = Image::new(1, 4, 'A');
        img.pixels_indexes = vec![RLE::new(2, 'A'), RLE::new(2, 'C')];
        img.add_interval(0, 3, 'B');
        assert_eq!(img.pixels_indexes, vec![RLE::new(4, 'B')]);
    }

    #[test]
    fn image_draw_rectangle_test() {
        let mut img = Image::new(10, 10, 'A');
        img.draw_rectangle(Position { x: 0, y: 0 }, Position { x: 4, y: 4 }, 'B');
        img.draw_rectangle(Position { x: 5, y: 5 }, Position { x: 9, y: 9 }, 'B');

        let mut expected = Vec::with_capacity(20);
        for _ in 0..5 {
            expected.push(RLE::new(5, 'B'));
            expected.push(RLE::new(5, 'A'));
        }

        let last_index = expected.len() - 1;
        expected[last_index] = RLE::new(10, 'A');
        expected.push(RLE::new(5, 'B'));

        for _ in 0..4 {
            expected.push(RLE::new(5, 'A'));
            expected.push(RLE::new(5, 'B'));
        }

        // DEBUG
        // for i in 0..img.pixels_indexes.len() {
        //     println!(
        //         "left: {:?}, right: {:?}",
        //         expected[i], img.pixels_indexes[i]
        //     );
        // }
        // let mut img_expected = Image::new(10, 10, 'A');
        // img_expected.pixels_indexes = expected.clone();
        // img.print_content();
        // img_expected.print_content();

        assert_eq!(expected, img.pixels_indexes);
    }
}
