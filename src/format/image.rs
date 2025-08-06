use std::{
    cmp::{max, min},
    fmt::{Debug, Display},
};

use crate::common::{
    text::{FontSize, get_or_init_text},
    types::{IPosition, Position},
};

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

    /// Draw text
    fn draw_text(
        &mut self,
        letter: &str,
        font_size: FontSize,
        top_left: IPosition,
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
    T: Clone + Copy + Ord + Display + Debug,
{
    pub fn new(height: usize, width: usize, default_color: T) -> Self {
        Image {
            height,
            width,
            pixels_indexes: vec![RLE::new((width * height) as usize, default_color.clone())],
        }
    }

    /// Merge adjacent entries of the pixel_indexes when they have the same values
    fn coalesce(&mut self) {
        let mut result: Vec<RLE<T>> = Vec::with_capacity(self.pixels_indexes.len());

        for rle in &self.pixels_indexes {
            if let Some(lst) = result.last_mut() {
                if lst.value == rle.value {
                    lst.freq += rle.freq;
                    continue;
                }
            }

            result.push(rle.clone());
        }

        self.pixels_indexes = result;
    }

    fn add_interval(&mut self, interval_start: usize, interval_end: usize, color: T) {
        let freq = interval_end - interval_start + 1;

        // Indexes of when an interval start or end is found
        let (mut index_start, mut index_end) = (0, 0);
        // current interval bounds (eg. [1, 4]);
        let (mut start_count, mut end_count) = (0, 0);

        let mut current_index = 0;
        let mut new_entries = vec![RLE::new(freq, color)];

        // For very specific cases
        let mut same_interval_when_end_reached = false;

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
            if start_count <= interval_end && interval_end <= end_count {
                index_end = current_index;

                // We do not forget to update the index start in case it is never touched
                if interval_start == interval_end {
                    same_interval_when_end_reached = true;
                }

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
        // println!("index_start: {index_start}, index_end: {index_end}");
        // println!("new_entries: {new_entries:?}");
        // println!();

        // Adding the new entries and replacing the one changed
        if same_interval_when_end_reached && new_entries.len() == 1 {
            self.pixels_indexes
                .insert(index_end + 1, new_entries[0].clone());
        } else {
            self.pixels_indexes
                .splice(index_start..index_end + 1, new_entries);
        }
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

    pub fn draw_text(
        &mut self,
        text: &str,
        font_size: FontSize,
        top_left: IPosition,
        color: T,
    ) -> &mut Self {
        let chars = text.chars();
        let font_size_2 = font_size.get_value() as isize;
        let spacing = font_size_2 / 3;

        let mut current_offset = 0;
        // NOTE could be done in parallel
        for c in chars {
            let mut current_top_left = top_left;
            current_top_left.x += current_offset;

            self.draw_letter(c, font_size, current_top_left, color);

            current_offset += spacing + font_size_2;
        }

        self.coalesce();

        self
    }

    pub fn draw_letter(
        &mut self,
        letter: char,
        font_size: FontSize,
        top_left: IPosition,
        color: T,
    ) -> &mut Self {
        // Fetch text to reuse already instanciated letter size
        let mut text = get_or_init_text().lock().unwrap();
        let letter = text.get_letter(letter, font_size);

        let (letter_h, letter_w) = (letter.len(), letter[0].len());

        // TODO factorize the following to use with other function
        let (x_start, x_end) = (
            max(0, top_left.x) as usize,
            min(self.width as isize, top_left.x + letter_w as isize) as usize,
        );

        let (y_start, y_end) = (
            max(0, top_left.y) as usize,
            min(self.height as isize, top_left.y + letter_h as isize) as usize,
        );

        // get the offset for the letter if the beginning of the letter is off canvas
        let (letter_x_offset, letter_y_offset) = (
            (x_start as isize - top_left.x).abs() as usize,
            (y_start as isize - top_left.y).abs() as usize,
        );

        for (i, y) in (y_start..y_end).enumerate() {
            let y = y * self.width;
            for (j, x) in (x_start..x_end).enumerate() {
                let interval = y + x;
                if letter[i + letter_y_offset][j + letter_x_offset] == 1 {
                    self.add_interval(interval, interval, color);
                }
            }
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
    fn image_add_interval_start_is_end() {
        // [(4, 'A')] ==> [(1, 'B'), (3, 'A')]
        let mut img = Image::new(1, 4, 'A');
        img.add_interval(0, 0, 'B');
        assert_eq!(img.pixels_indexes, vec![RLE::new(1, 'B'), RLE::new(3, 'A')]);

        // [(4, 'A')] ==> [(3, 'A'), (1, 'B')]
        let mut img = Image::new(1, 4, 'A');
        img.add_interval(3, 3, 'B');
        assert_eq!(img.pixels_indexes, vec![RLE::new(3, 'A'), RLE::new(1, 'B')]);

        // [(4, 'A')] ==> [(1, 'A'), (1, 'B'), (2, 'A')]
        let mut img = Image::new(1, 4, 'A');
        img.add_interval(1, 1, 'B');
        assert_eq!(
            img.pixels_indexes,
            vec![RLE::new(1, 'A'), RLE::new(1, 'B'), RLE::new(2, 'A')]
        );

        // [(1, 'A'), (1, 'B'), (4, 'C')] ==> [(1, 'A'), (1, 'B'), (1, 'D'), (3, 'C')]
        let mut img = Image::new(1, 6, 'A');
        img.pixels_indexes = vec![RLE::new(1, 'A'), RLE::new(1, 'B'), RLE::new(4, 'C')];
        img.add_interval(2, 2, 'D');
        assert_eq!(
            img.pixels_indexes,
            vec![
                RLE::new(1, 'A'),
                RLE::new(1, 'B'),
                RLE::new(1, 'D'),
                RLE::new(3, 'C')
            ]
        );
    }
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

    #[test]
    fn coalesce_test() {
        // [(4, 'A')] ==> [(4, 'A')]
        let mut img = Image::new(1, 4, 'A');
        img.pixels_indexes = vec![RLE::new(4, 'A')];
        img.coalesce();
        assert_eq!(img.pixels_indexes, vec![RLE::new(4, 'A')]);

        // [(2, 'A'), (2, 'A')] ==> [(4, 'A')]
        let mut img = Image::new(1, 4, 'A');
        img.pixels_indexes = vec![RLE::new(2, 'A'), RLE::new(2, 'A')];
        img.coalesce();
        assert_eq!(img.pixels_indexes, vec![RLE::new(4, 'A')]);

        // [(2, 'A'), (2, 'A'), (2, 'A'),(2, 'B'), (2, 'B') ] ==> [(6, 'A'), (4, B)]
        let mut img = Image::new(1, 4, 'A');
        img.pixels_indexes = vec![
            RLE::new(2, 'A'),
            RLE::new(2, 'A'),
            RLE::new(2, 'A'),
            RLE::new(2, 'B'),
            RLE::new(2, 'B'),
        ];
        img.coalesce();
        assert_eq!(img.pixels_indexes, vec![RLE::new(6, 'A'), RLE::new(4, 'B')]);
    }
}
