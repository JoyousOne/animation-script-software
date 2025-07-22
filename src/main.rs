use crate::{
    common::types::{Pixel, Position},
    format::gif::gif::{Gif, Loop},
};

mod common;
mod format;

fn main() {
    let (height, width) = (10u16, 10u16);
    let mut gif = Gif::new(height, width, Some(Loop::Forever));

    let white = Pixel::new(255, 255, 255);
    let red = Pixel::new(255, 0, 0);
    let blue = Pixel::new(0, 0, 255);
    // let black = Pixel::new(0, 0, 0);

    gif.add_image()
        .add_delay(10)
        .fill(&red)
        .draw_rectangle(Position::new(5, 0), Position::new(10, 5), &blue)
        .draw_rectangle(Position::new(0, 5), Position::new(5, 10), &blue)
        .draw_rectangle(Position::new(3, 3), Position::new(7, 7), &white);

    gif.add_image()
        .add_delay(10)
        .fill(&blue)
        .draw_rectangle(Position::new(5, 0), Position::new(10, 5), &red)
        .draw_rectangle(Position::new(0, 5), Position::new(5, 10), &red)
        .draw_rectangle(Position::new(3, 3), Position::new(7, 7), &white);

    let filename = "./test.gif";
    let _ = gif.write_to_file(filename);
}
