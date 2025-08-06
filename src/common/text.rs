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

pub const A: DefaultLetter = DefaultLetter {
    bitmap: [
        [0, 1, 1, 1, 0], // .###.
        [1, 0, 0, 0, 1], // #...#
        [1, 1, 1, 1, 1], // #####
        [1, 0, 0, 0, 1], // #...#
        [1, 0, 0, 0, 1], // #...#
    ],
};

type Letter = Vec<Vec<u8>>;

pub struct Text {
    letters_in_memory: HashMap<(char, usize), Letter>,
}

impl Text {
    pub fn new() -> Self {
        Text {
            letters_in_memory: HashMap::from([(('A', DEFAULT_FONT_SIZE), A.to_vec2d())]),
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
