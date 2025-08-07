use crate::{
    common::types::{Pixel, Position},
    format::{
        gif::gif::{Gif, Loop},
        image::ImageFormat,
    },
};

mod common;
mod format;

#[rustfmt::skip]
const BLACK: Pixel = Pixel {r: 0, g: 0, b: 0,};
#[rustfmt::skip]
const RED: Pixel = Pixel {r: 255, g: 0, b: 0,};
#[rustfmt::skip]
const BLUE: Pixel = Pixel {r: 0, g: 0, b: 255,};
#[rustfmt::skip]
const WHITE: Pixel = Pixel {r: 255, g: 255, b: 255,};

fn main() {
    // let (height, width) = (10u16, 10u16);
    let (height, width) = (80u16, 120u16);
    let mut gif = Gif::new(height, width, Some(Loop::Forever));

    // example1(&mut gif, height, width);
    // example2(&mut gif, height, width);
    // example3(&mut gif, height, width);
    // example_text();

    // DEBUG
    // gif.debug(None);

    // let filename = "./lol.gif";
    // let filename = "./test.gif";
    // let _ = gif.write_to_file(filename);
}

fn example1(gif: &mut Gif, height: usize, width: usize) {
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
            &RED,
        )
        .draw_rectangle(
            Position::new(0, height / 2),
            Position::new(width / 2, height),
            &RED,
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

fn example_text() {
    let (height, width) = (80u16, 120u16);
    let mut gif = Gif::new(height, width, Some(Loop::Forever));

    gif.add_image()
        .fill(&WHITE)
        .draw_text(
            "ABCDEFGHIJ",
            common::text::FontSize::Default,
            common::types::IPosition { x: 2, y: 5 },
            &RED,
        )
        .draw_text(
            "KMNOPQRSTU",
            common::text::FontSize::Default,
            common::types::IPosition { x: 2, y: 12 },
            &RED,
        )
        .draw_text(
            "VWXYZ",
            common::text::FontSize::Default,
            common::types::IPosition { x: 2, y: 19 },
            &RED,
        )
        // Min
        .draw_text(
            "abcdefghij",
            common::text::FontSize::Default,
            // common::text::FontSize::Size(8),
            common::types::IPosition { x: 2, y: 26 },
            &RED,
        )
        .draw_text(
            "klmnopqrstu",
            common::text::FontSize::Default,
            // common::text::FontSize::Size(8),
            common::types::IPosition { x: 2, y: 33 },
            &RED,
        )
        .draw_text(
            "vwxyz",
            common::text::FontSize::Default,
            // common::text::FontSize::Size(8),
            common::types::IPosition { x: 2, y: 40 },
            &RED,
        )
        .draw_text(
            "0123456789",
            common::text::FontSize::Default,
            // common::text::FontSize::Size(8),
            common::types::IPosition { x: 2, y: 47 },
            &RED,
        )
        .draw_text(
            "()[]{}+-*=% / \\ \" ' #@",
            common::text::FontSize::Default,
            // common::text::FontSize::Size(8),
            common::types::IPosition { x: 2, y: 54 },
            &RED,
        )
        .draw_text(
            "_&a,a.;:?!|<>^~",
            common::text::FontSize::Default,
            // common::text::FontSize::Size(8),
            common::types::IPosition { x: 2, y: 64 },
            &RED,
        );

    let filename = "./text.gif";
    let _ = gif.write_to_file(filename);
}
