use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

const DEFAULT_FONT_SIZE: usize = 5;

#[repr(usize)]
#[derive(Clone, Copy)]
pub enum FontSize {
    Default = DEFAULT_FONT_SIZE,
    Size(usize),
}

impl FontSize {
    pub fn get_value(&self) -> usize {
        match self {
            FontSize::Default => DEFAULT_FONT_SIZE,
            FontSize::Size(size) => *size,
        }
    }
}

struct DefaultLetter {
    bitmap: [[u8; DEFAULT_FONT_SIZE]; DEFAULT_FONT_SIZE],
}

impl DefaultLetter {
    pub fn to_vec2d(&self) -> Vec<Vec<u8>> {
        self.bitmap.iter().map(|x| x.to_vec()).collect()
    }
}

// Font inspired by https://www.dafont.com/minecraft.charmap?back=bitmap

const A: DefaultLetter = DefaultLetter {
    bitmap: [
        [0, 1, 1, 1, 0], // .###.
        [1, 0, 0, 0, 1], // #...#
        [1, 1, 1, 1, 1], // #####
        [1, 0, 0, 0, 1], // #...#
        [1, 0, 0, 0, 1], // #...#
    ],
};

const B: DefaultLetter = DefaultLetter {
    bitmap: [
        [1, 1, 1, 1, 0], // ####.
        [1, 0, 0, 0, 1], // #...#
        [1, 1, 1, 1, 0], // ####.
        [1, 0, 0, 0, 1], // #...#
        [1, 1, 1, 1, 0], // ####.
    ],
};

const C: DefaultLetter = DefaultLetter {
    bitmap: [
        [0, 1, 1, 1, 0], // .###.
        [1, 0, 0, 0, 1], // #...#
        [1, 0, 0, 0, 0], // #....
        [1, 0, 0, 0, 1], // #...#
        [0, 1, 1, 1, 0], // .###.
    ],
};

const D: DefaultLetter = DefaultLetter {
    bitmap: [
        [1, 1, 1, 1, 0], // ####.
        [1, 0, 0, 0, 1], // #...#
        [1, 0, 0, 0, 1], // #...#
        [1, 0, 0, 0, 1], // #...#
        [1, 1, 1, 1, 0], // ####.
    ],
};

const E: DefaultLetter = DefaultLetter {
    bitmap: [
        [1, 1, 1, 1, 1], // #####
        [1, 0, 0, 0, 0], // #....
        [1, 1, 1, 1, 0], // #####
        [1, 0, 0, 0, 0], // #....
        [1, 1, 1, 1, 1], // #####
    ],
};

const F: DefaultLetter = DefaultLetter {
    bitmap: [
        [1, 1, 1, 1, 1], // #####
        [1, 0, 0, 0, 0], // #....
        [1, 1, 1, 1, 0], // ####.
        [1, 0, 0, 0, 0], // #....
        [1, 0, 0, 0, 0], // #....
    ],
};

const G: DefaultLetter = DefaultLetter {
    bitmap: [
        [0, 1, 1, 1, 1], // .####
        [1, 0, 0, 0, 0], // #....
        [1, 0, 1, 1, 1], // #.###
        [1, 0, 0, 0, 1], // #...#
        [0, 1, 1, 1, 0], // .###.
    ],
};

const H: DefaultLetter = DefaultLetter {
    bitmap: [
        [1, 0, 0, 0, 1], // #...#
        [1, 0, 0, 0, 1], // #...#
        [1, 1, 1, 1, 1], // #####
        [1, 0, 0, 0, 1], // #...#
        [1, 0, 0, 0, 1], // #...#
    ],
};

const I: DefaultLetter = DefaultLetter {
    bitmap: [
        [1, 1, 1, 1, 1], // #####
        [0, 0, 1, 0, 0], // ..#..
        [0, 0, 1, 0, 0], // ..#..
        [0, 0, 1, 0, 0], // ..#..
        [1, 1, 1, 1, 1], // #####
    ],
};

const J: DefaultLetter = DefaultLetter {
    bitmap: [
        [0, 0, 0, 1, 0], // ...#.
        [0, 0, 0, 1, 0], // ...#.
        [0, 0, 0, 1, 0], // ...#.
        [1, 0, 0, 1, 0], // #..#.
        [0, 1, 1, 1, 0], // .###.
    ],
};

const K: DefaultLetter = DefaultLetter {
    bitmap: [
        [1, 0, 0, 0, 1], // #...#
        [1, 0, 0, 1, 0], // #..#.
        [1, 1, 1, 0, 0], // ###..
        [1, 0, 0, 1, 0], // #..#.
        [1, 0, 0, 0, 1], // #...#
    ],
};

const L: DefaultLetter = DefaultLetter {
    bitmap: [
        [1, 0, 0, 0, 0], // #....
        [1, 0, 0, 0, 0], // #....
        [1, 0, 0, 0, 0], // #....
        [1, 0, 0, 0, 0], // #....
        [1, 1, 1, 1, 1], // #####
    ],
};

const M: DefaultLetter = DefaultLetter {
    bitmap: [
        [1, 0, 0, 0, 1], // #...#
        [1, 1, 0, 1, 1], // ##.##
        [1, 0, 1, 0, 1], // #.#.#
        [1, 0, 0, 0, 1], // #...#
        [1, 0, 0, 0, 1], // #...#
    ],
};

const N: DefaultLetter = DefaultLetter {
    bitmap: [
        [1, 0, 0, 0, 1], // #...#
        [1, 1, 0, 0, 1], // ##..#
        [1, 0, 1, 0, 1], // #.#.#
        [1, 0, 0, 1, 1], // #..##
        [1, 0, 0, 0, 1], // #...#
    ],
};

const O: DefaultLetter = DefaultLetter {
    bitmap: [
        [0, 1, 1, 1, 0], // .###.
        [1, 0, 0, 0, 1], // #...#
        [1, 0, 0, 0, 1], // #...#
        [1, 0, 0, 0, 1], // #...#
        [0, 1, 1, 1, 0], // .###.
    ],
};

const P: DefaultLetter = DefaultLetter {
    bitmap: [
        [1, 1, 1, 1, 0], // ####.
        [1, 0, 0, 0, 1], // #...#
        [1, 1, 1, 1, 0], // ####.
        [1, 0, 0, 0, 0], // #....
        [1, 0, 0, 0, 0], // #....
    ],
};

const Q: DefaultLetter = DefaultLetter {
    bitmap: [
        [0, 1, 1, 0, 0], // .##..
        [1, 0, 0, 1, 0], // #..#.
        [1, 0, 0, 1, 0], // #..#.
        [1, 0, 0, 1, 0], // #..#.
        [0, 1, 1, 1, 1], // .####
    ],
};

const R: DefaultLetter = DefaultLetter {
    bitmap: [
        [1, 1, 1, 1, 0], // ####.
        [1, 0, 0, 0, 1], // #...#
        [1, 1, 1, 1, 0], // ####.
        [1, 0, 1, 0, 0], // #.#..
        [1, 0, 0, 1, 0], // #..#.
    ],
};

const S: DefaultLetter = DefaultLetter {
    bitmap: [
        [0, 1, 1, 1, 1], // .####
        [1, 0, 0, 0, 0], // #....
        [0, 1, 1, 1, 0], // .###.
        [0, 0, 0, 0, 1], // ....#
        [1, 1, 1, 1, 0], // ####.
    ],
};

const T: DefaultLetter = DefaultLetter {
    bitmap: [
        [1, 1, 1, 1, 1], // #####
        [0, 0, 1, 0, 0], // ..#..
        [0, 0, 1, 0, 0], // ..#..
        [0, 0, 1, 0, 0], // ..#..
        [0, 0, 1, 0, 0], // ..#..
    ],
};

const U: DefaultLetter = DefaultLetter {
    bitmap: [
        [1, 0, 0, 0, 1], // #...#
        [1, 0, 0, 0, 1], // #...#
        [1, 0, 0, 0, 1], // #...#
        [1, 0, 0, 0, 1], // #...#
        [0, 1, 1, 1, 0], // .###.
    ],
};

const V: DefaultLetter = DefaultLetter {
    bitmap: [
        [1, 0, 0, 0, 1], // #...#
        [1, 0, 0, 0, 1], // #...#
        [0, 1, 0, 1, 0], // .#.#.
        [0, 1, 0, 1, 0], // .#.#.
        [0, 0, 1, 0, 0], // ..#..
    ],
};

const W: DefaultLetter = DefaultLetter {
    bitmap: [
        [1, 0, 0, 0, 1], // #...#
        [1, 0, 0, 0, 1], // #...#
        [1, 0, 1, 0, 1], // #.#.#
        [1, 1, 0, 1, 1], // ##.##
        [1, 0, 0, 0, 1], // #...#
    ],
};

const X: DefaultLetter = DefaultLetter {
    bitmap: [
        [1, 0, 0, 0, 1], // #...#
        [0, 1, 0, 1, 0], // .#.#.
        [0, 0, 1, 0, 0], // ..#..
        [0, 1, 0, 1, 0], // .#.#.
        [1, 0, 0, 0, 1], // #...#
    ],
};

const Y: DefaultLetter = DefaultLetter {
    bitmap: [
        [1, 0, 0, 0, 1], // #...#
        [0, 1, 0, 1, 0], // .#.#.
        [0, 0, 1, 0, 0], // ..#..
        [0, 0, 1, 0, 0], // ..#..
        [0, 0, 1, 0, 0], // ..#..
    ],
};

const Z: DefaultLetter = DefaultLetter {
    bitmap: [
        [1, 1, 1, 1, 1], // #####
        [0, 0, 0, 1, 0], // ...#.
        [0, 0, 1, 0, 0], // ..#..
        [0, 1, 0, 0, 0], // .#...
        [1, 1, 1, 1, 1], // #####
    ],
};

const _A: DefaultLetter = DefaultLetter {
    bitmap: [
        [0, 0, 0, 0, 0], // .....
        [0, 0, 0, 0, 0], // .....
        [0, 0, 0, 0, 0], // .....
        [0, 0, 0, 0, 0], // .....
        [0, 0, 0, 0, 0], // .....
    ],
};

type Letter = Vec<Vec<u8>>;

pub struct Text {
    letters_in_memory: HashMap<(char, usize), Letter>,
}

impl Text {
    pub fn new() -> Self {
        Text {
            letters_in_memory: HashMap::from([
                (('A', DEFAULT_FONT_SIZE), A.to_vec2d()),
                (('B', DEFAULT_FONT_SIZE), B.to_vec2d()),
                (('C', DEFAULT_FONT_SIZE), C.to_vec2d()),
                (('D', DEFAULT_FONT_SIZE), D.to_vec2d()),
                (('E', DEFAULT_FONT_SIZE), E.to_vec2d()),
                (('F', DEFAULT_FONT_SIZE), F.to_vec2d()),
                (('G', DEFAULT_FONT_SIZE), G.to_vec2d()),
                (('H', DEFAULT_FONT_SIZE), H.to_vec2d()),
                (('I', DEFAULT_FONT_SIZE), I.to_vec2d()),
                (('J', DEFAULT_FONT_SIZE), J.to_vec2d()),
                (('K', DEFAULT_FONT_SIZE), K.to_vec2d()),
                (('L', DEFAULT_FONT_SIZE), L.to_vec2d()),
                (('M', DEFAULT_FONT_SIZE), M.to_vec2d()),
                (('N', DEFAULT_FONT_SIZE), N.to_vec2d()),
                (('O', DEFAULT_FONT_SIZE), O.to_vec2d()),
                (('P', DEFAULT_FONT_SIZE), P.to_vec2d()),
                (('Q', DEFAULT_FONT_SIZE), Q.to_vec2d()),
                (('R', DEFAULT_FONT_SIZE), R.to_vec2d()),
                (('S', DEFAULT_FONT_SIZE), S.to_vec2d()),
                (('T', DEFAULT_FONT_SIZE), T.to_vec2d()),
                (('U', DEFAULT_FONT_SIZE), U.to_vec2d()),
                (('V', DEFAULT_FONT_SIZE), V.to_vec2d()),
                (('W', DEFAULT_FONT_SIZE), W.to_vec2d()),
                (('X', DEFAULT_FONT_SIZE), X.to_vec2d()),
                (('Y', DEFAULT_FONT_SIZE), Y.to_vec2d()),
                (('Z', DEFAULT_FONT_SIZE), Z.to_vec2d()),
            ]),
        }
    }

    pub fn get_letter(&mut self, letter: char, font_size: FontSize) -> &Letter {
        let font_size = font_size.get_value();

        assert!(
            font_size >= DEFAULT_FONT_SIZE,
            "The font size cannot be bellow the default font size: {DEFAULT_FONT_SIZE}",
        );

        if self.letters_in_memory.contains_key(&(letter, font_size)) {
            return self.letters_in_memory.get(&(letter, font_size)).unwrap();
        }

        self.add_new_letter_format(letter, font_size)
    }

    fn add_new_letter_format(&mut self, letter: char, font_size: usize) -> &Letter {
        // TODO add match expression to handle not added char
        let src = self
            .letters_in_memory
            .get(&(letter, DEFAULT_FONT_SIZE))
            .unwrap();

        // Creating empty bitmap
        let mut new_bitmap = Vec::with_capacity(font_size);
        for _ in 0..font_size {
            new_bitmap.push(vec![0; font_size]);
        }

        let src_h = src.len();
        let src_w = src_h; // TODO may differ later

        let res_h = font_size;
        let res_w = res_h; // TODO may differ later

        for y in 0..res_h {
            let src_y = y * src_h / res_h;
            for x in 0..res_w {
                let src_x = x * src_w / res_w;
                new_bitmap[y][x] = src[src_y][src_x];
            }
        }

        self.letters_in_memory
            .insert((letter, font_size), new_bitmap);

        self.letters_in_memory.get(&(letter, font_size)).unwrap()
    }
}

// Global mutable variable to reuse already instanciated letters
static TEXT: OnceLock<Mutex<Text>> = OnceLock::new();

pub fn get_or_init_text() -> &'static Mutex<Text> {
    TEXT.get_or_init(|| Mutex::new(Text::new()))
}

// DEBUG
// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn size_change() {
//         let pretty_print = |xs: &Vec<Vec<u8>>| {
//             for x in xs {
//                 println!("{x:?}")
//             }
//             println!();
//         };

//         let mut text = Text::new();

//         let a5 = text.get_letter('A', FontSize::Default);
//         println!("a taille 5: ");
//         pretty_print(a5);

//         let a6 = text.get_letter('A', FontSize::Size(6));
//         println!("a taille 6: ");
//         pretty_print(a6);

//         let a7 = text.get_letter('A', FontSize::Size(7));
//         println!("a taille 7: ");
//         pretty_print(a7);

//         let a8 = text.get_letter('A', FontSize::Size(8));
//         println!("a taille 8: ");
//         pretty_print(a8);

//         let a9 = text.get_letter('A', FontSize::Size(9));
//         println!("a taille 9: ");
//         pretty_print(a9);

//         let a16 = text.get_letter('A', FontSize::Size(16));
//         println!("a taille 16: ");
//         pretty_print(a16);
//     }
// }
