use crate::common::types::Color;
use crate::format::gif::gif_image::Image;
use std::{cell::RefCell, cmp::min, fs::File, io::Write, rc::Rc};

pub const SIGNATURE: [u8; 3] = *b"GIF";
pub const VERSION: [u8; 3] = *b"89a";
pub const TRAILER_MARKER: u8 = 0x3B;

pub struct Gif {
    height: u16,
    width: u16,
    global_color_table: Rc<RefCell<Vec<Color>>>,
    images: Vec<Image>, // TODO add image type
}

impl Gif {
    pub fn new(height: u16, width: u16, use_global_color_table: bool) -> Self {
        Gif {
            height,
            width,
            global_color_table: Rc::new(RefCell::new(Vec::new())),
            images: Vec::new(),
        }
    }

    fn get_global_color_table_size_bits(&self) -> u8 {
        let color_count = self.global_color_table.borrow().len();
        let bit_size = ((color_count as f64).log2().ceil() - 1.0) as u8;

        min(bit_size, 0x07)
    }

    // TODO for now only global table
    pub fn add_image(&mut self) -> &mut Image {
        let new_image = Image::new(
            self.height,
            self.width,
            Some(Rc::clone(&self.global_color_table)),
        );

        self.images.push(new_image);

        self.images.last_mut().unwrap()
    }

    pub fn write_to_file(&mut self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
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
        {
            let mut global_color_table = self.global_color_table.borrow_mut();

            // Filling global color table with padding
            let padding_size = (1 << (global_color_bits + 1)) - global_color_table.len();
            for _ in 0..padding_size {
                global_color_table.push(Color::new(0, 0, 0));
            }

            // add colors
            for color in global_color_table.iter() {
                file_contents.extend(color.as_bytes());
            }
        }

        // add images
        for image in &self.images {
            file_contents.extend_from_slice(&image.as_bytes());
        }

        // add gif end
        file_contents.push(TRAILER_MARKER);

        // DEBUG
        // println!("file_contents: {:02X?}", file_contents);

        // write to file
        let mut file = File::create(filename)?;
        file.write_all(&file_contents)?;

        Ok(())
    }
}
