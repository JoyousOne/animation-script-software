use std::{
    cmp::max,
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

pub fn to_vec2d<const N: usize>(bitmap: &'static [[u8; N]]) -> Vec<Vec<u8>> {
    bitmap.iter().map(|row| row.to_vec()).collect()
}

const A_BITMAP: &[[u8; 5]] = &[
    [0, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
];

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

/** Min chars **/

const MIN_A_BITMAP: &[[u8; 4]] = &[
    [0, 1, 1, 0], // .##.
    [0, 0, 0, 1], // ...#
    [0, 1, 1, 1], // .###
    [1, 0, 0, 1], // #..#
    [0, 1, 1, 1], // .###
];

const MIN_B_BITMAP: &[[u8; 5]] = &[
    [1, 0, 0, 0, 0], // #....
    [1, 0, 1, 1, 0], // #.##.
    [1, 1, 0, 0, 1], // ##..#
    [1, 0, 0, 0, 1], // #...#
    [1, 1, 1, 1, 0], // ####.
];

const MIN_C_BITMAP: &[[u8; 4]] = &[
    [0, 0, 0, 0], // .....
    [0, 1, 1, 1], // .####
    [1, 0, 0, 0], // #....
    [1, 0, 0, 0], // #....
    [0, 1, 1, 1], // .####
];

const MIN_D_BITMAP: &[[u8; 4]] = &[
    [0, 0, 0, 1], // .....
    [0, 1, 1, 1], // .###.
    [1, 0, 0, 1], // #....
    [1, 0, 0, 1], // #....
    [1, 1, 1, 1], // .###.
];

const MIN_E_BITMAP: &[[u8; 4]] = &[
    // [0, 0, 0, 0, 0], // .....
    [0, 1, 1, 1], // .###
    [1, 0, 0, 1], // #..#
    [1, 1, 1, 0], // ###.
    [1, 0, 0, 0], // #...
    [0, 1, 1, 1], // .###
];

const MIN_F_BITMAP: &[[u8; 4]] = &[
    [0, 1, 1, 0], // .##.
    [0, 1, 0, 0], // .#..
    [1, 1, 1, 1], // ####
    [0, 1, 0, 0], // .#..
    [0, 1, 0, 0], // .#..
];

const MIN_G_BITMAP: &[[u8; 4]] = &[
    [0, 0, 0, 0], // ....
    [0, 1, 1, 1], // .###
    [1, 0, 0, 1], // #..#
    [0, 1, 1, 1], // .###
    [0, 0, 0, 1], // ...#
    [1, 0, 0, 1], // #..#
    [0, 1, 1, 0], // .##.
];

const MIN_H_BITMAP: &[[u8; 4]] = &[
    [1, 0, 0, 0], // #...
    [1, 0, 0, 0], // #...
    [1, 0, 1, 0], // #.#.
    [1, 1, 0, 1], // ##.#
    [1, 0, 0, 1], // #..#
];

// FIXME problematic when size differ
const MIN_I_BITMAP: &[[u8; 1]] = &[
    [1], // #
    [0], // .
    [1], // #
    [1], // #
    [1], // #
];

const MIN_J_BITMAP: &[[u8; 4]] = &[
    [0, 0, 0, 1], // ...#
    [0, 0, 0, 0], // ....
    [0, 0, 0, 1], // ...#
    [0, 0, 0, 1], // ...#
    [1, 0, 0, 1], // #..#
    [0, 1, 1, 1], // .###
];

const MIN_K_BITMAP: &[[u8; 4]] = &[
    [1, 0, 0, 1], // #..#
    [1, 0, 1, 0], // #.#.
    [1, 1, 0, 0], // ##..
    [1, 0, 1, 0], // #.#.
    [1, 0, 0, 1], // #..#
];

// FIXME problematic when size differ
const MIN_L_BITMAP: &[[u8; 1]] = &[
    [1], // #
    [1], // #
    [1], // #
    [1], // #
    [1], // #
];

const MIN_M_BITMAP: &[[u8; 5]] = &[
    [0, 0, 0, 0, 0], // .....
    [1, 1, 0, 1, 0], // ##.#.
    [1, 0, 1, 0, 1], // #.#.#
    [1, 0, 1, 0, 1], // #.#.#
    [1, 0, 1, 0, 1], // #.#.#
];

const MIN_N_BITMAP: &[[u8; 4]] = &[
    [0, 0, 0, 0], // ....
    [1, 1, 1, 0], // ###.
    [1, 0, 0, 1], // #..#
    [1, 0, 0, 1], // #..#
    [1, 0, 0, 1], // #..#
];

const MIN_O_BITMAP: &[[u8; 4]] = &[
    [0, 0, 0, 0], // ....
    [0, 1, 1, 0], // .##.
    [1, 0, 0, 1], // #..#
    [1, 0, 0, 1], // #..#
    [0, 1, 1, 0], // .##.
];

const MIN_P_BITMAP: &[[u8; 4]] = &[
    [0, 0, 0, 0], // .....
    [1, 1, 1, 0], // ###..
    [1, 0, 0, 1], // #..#.
    [1, 1, 1, 1], // ####.
    [1, 0, 0, 0], // #....
    [1, 0, 0, 0], // #....
];

const MIN_Q_BITMAP: &[[u8; 4]] = &[
    [0, 0, 0, 0], // .....
    [1, 1, 1, 1], // #####
    [1, 0, 0, 1], // #...#
    [0, 1, 1, 1], // .####
    [0, 0, 0, 1], // ....#
    [0, 0, 0, 1], // ....#
];

const MIN_R_BITMAP: &[[u8; 4]] = &[
    [0, 0, 0, 0], // .....
    [1, 0, 1, 0], // #.#..
    [1, 1, 0, 1], // ##.#.
    [1, 0, 0, 0], // #....
    [1, 0, 0, 0], // #....
];

const MIN_S_BITMAP: &[[u8; 4]] = &[
    [0, 0, 0, 0], // ....
    [0, 0, 1, 1], // ..##
    [0, 1, 0, 0], // .#..
    [0, 0, 1, 0], // ..#.
    [1, 1, 0, 0], // ##..
];

const MIN_T_BITMAP: &[[u8; 3]] = &[
    [0, 1, 0], // .#.
    [1, 1, 1], // ###
    [0, 1, 0], // .#.
    [0, 1, 0], // .#.
    [0, 1, 1], // .##
];

const MIN_U_BITMAP: &[[u8; 4]] = &[
    [0, 0, 0, 0], // ....
    [1, 0, 0, 1], // #..#
    [1, 0, 0, 1], // #..#
    [1, 0, 0, 1], // #..#
    [0, 1, 1, 1], // .###
];

const MIN_V_BITMAP: &[[u8; 5]] = &[
    [0, 0, 0, 0, 0], // .....
    [1, 0, 0, 0, 1], // #...#
    [1, 0, 0, 0, 1], // #...#
    [0, 1, 0, 1, 0], // .#.#.
    [0, 0, 1, 0, 0], // ..#..
];

const MIN_W_BITMAP: &[[u8; 5]] = &[
    [0, 0, 0, 0, 0], // .....
    [1, 0, 0, 0, 1], // #...#
    [1, 0, 1, 0, 1], // #...#
    [1, 0, 1, 0, 1], // #.#.#
    [0, 1, 0, 1, 1], // .#.##
];

const MIN_X_BITMAP: &[[u8; 5]] = &[
    [0, 0, 0, 0, 0], // .....
    [1, 0, 0, 0, 1], // #...#
    [0, 1, 0, 1, 0], // .#.#.
    [0, 0, 1, 0, 0], // ..#..
    [1, 1, 0, 1, 1], // ##.##
];

const MIN_Y_BITMAP: &[[u8; 4]] = &[
    [0, 0, 0, 0], // ....
    [1, 0, 0, 1], // #..#
    [1, 0, 0, 1], // #..#
    [0, 1, 1, 1], // .###
    [0, 0, 0, 1], // ...#
    [1, 1, 1, 0], // ###.
];

const MIN_Z_BITMAP: &[[u8; 4]] = &[
    [0, 0, 0, 0], // ....
    [1, 1, 1, 1], // ####
    [0, 0, 1, 0], // ..#.
    [0, 1, 0, 0], // .#..
    [1, 1, 1, 1], // ####
];

// const _BITMAP: &[[u8; 5]] = &[
//     [0, 0, 0, 0, 0], // .....
//     [0, 0, 0, 0, 0], // .....
//     [0, 0, 0, 0, 0], // .....
//     [0, 0, 0, 0, 0], // .....
//     [0, 0, 0, 0, 0], // .....
// ];

const UNKNOWN_BITMAP: &[[u8; 5]] = &[
    [1, 1, 1, 1, 1], // #####
    [1, 0, 1, 0, 1], // #.#.#
    [1, 1, 0, 1, 1], // ##.##
    [1, 0, 1, 0, 1], // #.#.#
    [1, 1, 1, 1, 1], // #####
];

pub type Letter = Vec<Vec<u8>>;

pub struct Text {
    letters_in_memory: HashMap<(char, usize), Letter>,
}

impl Text {
    pub fn new() -> Self {
        Text {
            letters_in_memory: HashMap::from([
                // Capital
                (('A', DEFAULT_FONT_SIZE), to_vec2d(A_BITMAP)),
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
                // Min
                (('a', DEFAULT_FONT_SIZE), to_vec2d(MIN_A_BITMAP)),
                (('b', DEFAULT_FONT_SIZE), to_vec2d(MIN_B_BITMAP)),
                (('c', DEFAULT_FONT_SIZE), to_vec2d(MIN_C_BITMAP)),
                (('d', DEFAULT_FONT_SIZE), to_vec2d(MIN_D_BITMAP)),
                (('e', DEFAULT_FONT_SIZE), to_vec2d(MIN_E_BITMAP)),
                (('f', DEFAULT_FONT_SIZE), to_vec2d(MIN_F_BITMAP)),
                (('g', DEFAULT_FONT_SIZE), to_vec2d(MIN_G_BITMAP)),
                (('h', DEFAULT_FONT_SIZE), to_vec2d(MIN_H_BITMAP)),
                (('i', DEFAULT_FONT_SIZE), to_vec2d(MIN_I_BITMAP)),
                (('j', DEFAULT_FONT_SIZE), to_vec2d(MIN_J_BITMAP)),
                (('k', DEFAULT_FONT_SIZE), to_vec2d(MIN_K_BITMAP)),
                (('l', DEFAULT_FONT_SIZE), to_vec2d(MIN_L_BITMAP)),
                (('m', DEFAULT_FONT_SIZE), to_vec2d(MIN_M_BITMAP)),
                (('n', DEFAULT_FONT_SIZE), to_vec2d(MIN_N_BITMAP)),
                (('o', DEFAULT_FONT_SIZE), to_vec2d(MIN_O_BITMAP)),
                (('p', DEFAULT_FONT_SIZE), to_vec2d(MIN_P_BITMAP)),
                (('q', DEFAULT_FONT_SIZE), to_vec2d(MIN_Q_BITMAP)),
                (('r', DEFAULT_FONT_SIZE), to_vec2d(MIN_R_BITMAP)),
                (('t', DEFAULT_FONT_SIZE), to_vec2d(MIN_T_BITMAP)),
                (('s', DEFAULT_FONT_SIZE), to_vec2d(MIN_S_BITMAP)),
                (('u', DEFAULT_FONT_SIZE), to_vec2d(MIN_U_BITMAP)),
                (('v', DEFAULT_FONT_SIZE), to_vec2d(MIN_V_BITMAP)),
                (('w', DEFAULT_FONT_SIZE), to_vec2d(MIN_W_BITMAP)),
                (('x', DEFAULT_FONT_SIZE), to_vec2d(MIN_X_BITMAP)),
                (('y', DEFAULT_FONT_SIZE), to_vec2d(MIN_Y_BITMAP)),
                (('z', DEFAULT_FONT_SIZE), to_vec2d(MIN_Z_BITMAP)),
                // Numerical
                // Special character for unknown variables
                (('\0', DEFAULT_FONT_SIZE), to_vec2d(UNKNOWN_BITMAP)),
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
        let src = match self.letters_in_memory.get(&(letter, DEFAULT_FONT_SIZE)) {
            Some(existing) => existing,
            None => self
                .letters_in_memory
                .get(&('\0', DEFAULT_FONT_SIZE))
                .unwrap(),
        };

        // Creating empty bitmap
        let mut new_bitmap = Vec::with_capacity(font_size);
        for _ in 0..font_size {
            new_bitmap.push(vec![0; font_size]);
        }

        let src_h = src.len();
        let src_w = src[0].len();

        let diff_w = DEFAULT_FONT_SIZE - src[0].len();

        let res_h = font_size;
        let res_w = font_size - diff_w as usize; // TODO may differ later

        // if letter == 'a' {
        //     println!("diff_w: {diff_w}");
        //     println!("res_w: {res_w}");
        // }

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
