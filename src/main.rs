use crate::{
    common::types::{Pixel, Position},
    format::{
        gif::gif::{Gif, Loop},
        image::ImageFormat,
    },
};

mod common;
mod format;

fn main() {
    let (height, width) = (10u16, 10u16);
    let mut gif = Gif::new(height, width, Some(Loop::Forever));

    // example1(&mut gif, height, width);
    // example2(&mut gif, height, width);
    // example3(&mut gif, height, width);

    // DEBUG
    // gif.debug(None);

    let filename = "./test.gif";
    // let filename = "./chud.gif";
    let _ = gif.write_to_file(filename);
}

#[rustfmt::skip]
const BLACK: Pixel = Pixel {r: 0, g: 0, b: 0,};
#[rustfmt::skip]
const RED: Pixel = Pixel {r: 255, g: 0, b: 0,};
#[rustfmt::skip]
const BLUE: Pixel = Pixel {r: 0, g: 0, b: 255,};
#[rustfmt::skip]
const WHITE: Pixel = Pixel {r: 255, g: 255, b: 255,};

fn example1(gif: &mut Gif, height: usize, width: usize) {
    let white = Pixel::new(255, 255, 255);
    let red = Pixel::new(255, 0, 0);

    gif.add_image()
        .add_delay(100)
        .fill(&RED)
        .draw_rectangle(
            Position::new(width / 2, 0),
            Position::new(width, height / 2),
            &BLUE,
        )
        .draw_rectangle(
            Position::new(0, height / 2),
            Position::new(width / 2, height),
            &BLUE,
        )
        .draw_rectangle(
            Position::new(width / 3, height / 3),
            Position::new(width - (width / 3), height - (height / 3)),
            // Position::new((width as usize / 3) * 2, 7),
            &WHITE,
        );

    gif.add_image()
        .add_delay(100)
        .fill(&BLUE)
        .draw_rectangle(
            Position::new(width / 2, 0),
            Position::new(width, height / 2),
            &red,
        )
        .draw_rectangle(
            Position::new(0, height / 2),
            Position::new(width / 2, height),
            &red,
        )
        .draw_rectangle(
            Position::new(width / 3, height / 3),
            Position::new(width - (width / 3), height - (height / 3)),
            &WHITE,
        );
}

fn example2(gif: &mut Gif, height: usize, width: usize) {
    let delay = 10;
    let mut colors = vec![WHITE, RED, BLUE];
    // let mut colors = vec![red, BLUE];

    for _ in 0..10 {
        colors.rotate_right(1);
        let img = gif.add_image().add_delay(delay).fill(&colors[0]);
        for i in 1..10 {
            img.draw_rectangle(
                Position::new((width / 10) * i, (height / 10) * i),
                Position::new(width - (width / 10) * i, height - (height / 10) * i),
                &colors[i % colors.len()],
            );
        }
    }
}

fn example3(gif: &mut Gif, height: usize, width: usize) {
    // let img = gif.add_image().add_delay(50).fill(&RED);
    let img = gif.add_image().fill(&RED);

    let distance_from_border_x = width / 10;
    let distance_from_border_y = height / 14;
    let (size_x, size_y) = (120, 60);
    let border_size = 4;
    // left
    img.draw_rectangle(
        Position::new(distance_from_border_x, distance_from_border_y),
        Position::new(
            distance_from_border_x + size_x,
            distance_from_border_y + size_y,
        ),
        &BLACK,
    )
    .draw_rectangle(
        Position::new(
            distance_from_border_x + border_size,
            distance_from_border_y + border_size,
        ),
        Position::new(
            distance_from_border_x + size_x - border_size,
            distance_from_border_y + size_y - border_size,
        ),
        &WHITE,
    );

    // right
    img.draw_rectangle(
        Position::new(
            width - distance_from_border_x,
            height - distance_from_border_y,
        ),
        Position::new(
            width - distance_from_border_x + size_x,
            height - distance_from_border_y + size_y,
        ),
        &BLACK,
    )
    .draw_rectangle(
        Position::new(
            width - distance_from_border_x + border_size,
            height - distance_from_border_y + border_size,
        ),
        Position::new(
            width - distance_from_border_x + size_x - border_size,
            height - distance_from_border_y + size_y - border_size,
        ),
        &WHITE,
    );
}
