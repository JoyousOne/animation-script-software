use crate::common::types::{Color, Image};
use std::{cmp::min, fs::File, io::Write};

pub const SIGNATURE: [u8; 3] = *b"GIF";
pub const VERSION: [u8; 3] = *b"89a";
pub const TRAILER_MARKER: u8 = 0x3B;

const EXTENSIONS_INTRODUCER: u8 = 0x21;
const GRAPHIC_CONTROL_LABEL: u8 = 0xF9;

struct Gif {
    height: u16,
    width: u16,
    global_color_table: Vec<Color>,
    images: Vec<Image>, // TODO add image type
}

impl Gif {
    fn new(height: u16, width: u16, use_global_color_table: bool) -> Self {
        Gif {
            height,
            width,
            global_color_table: vec![],
            images: vec![],
        }
    }

    fn get_global_color_table_size_bits(&self) -> u8 {
        let color_count = self.global_color_table.len();
        let bit_size = ((color_count as f64).log2().ceil() - 1.0) as u8;

        min(bit_size, 0x07)
    }

    fn add_image() {
        // TODO
    }

    fn write_to_file(&mut self, filename: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut file_contents = vec![];
        let global_color_bits = self.get_global_color_table_size_bits();

        // add header block
        file_contents.extend_from_slice(&SIGNATURE);
        file_contents.extend_from_slice(&VERSION);

        // add logistical screen descriptor
        file_contents.extend_from_slice(&self.width.to_le_bytes());
        file_contents.extend_from_slice(&self.height.to_le_bytes());

        // add packed field
        let packed_field = 0x90 | global_color_bits;
        file_contents.push(packed_field);

        // add background color index
        file_contents.push(0);

        // add pixel aspect ratio
        file_contents.push(0);

        // add padding to the color table
        let padding_size = (1 << (global_color_bits + 1)) - self.global_color_table.len();

        for _ in 0..padding_size {
            self.global_color_table.push(Color { r: 0, g: 0, b: 0 });
        }

        // add colors
        for color in self.global_color_table {
            file_contents.extend(color.as_bytes());
        }

        // add images
        for image in self.images {
            file_contents.push(image.get_image_descriptor()); // TODO check with image code
            file_contents.push(image.get_image_data()); // TODO check with image code
        }

        // add gif end
        file_contents.push(TRAILER_MARKER);

        // write to file
        let file = File::create(filename)?;
        file.write_all(&file_contents)?;

        Ok(())
    }
}
