use std::{cell::RefCell, rc::Rc};

use crate::{
    common::{
        text::FontSize,
        types::{Color, IPosition, Pixel, Position},
    },
    format::image::{Image, ImageFormat},
};

/** Graphical Control Extension **/
const GRAPHICAL_CONTROL_EXTENSION_SIZE: usize = 8;
const EXTENSIONS_INTRODUCER: u8 = 0x21;
const GRAPHIC_CONTROL_LABEL: u8 = 0xF9;
const GCE_BYTE_SIZE: u8 = 0x04;

/** Image Descriptor **/
const IMAGE_DESCRIPTOR_SIZE: usize = 10;
const IMAGE_SEPARATOR: u8 = b'\x2C';

/** Image Data **/
const SUB_BLOCK_MAX_SIZE: usize = 255;
const CODE_TABLE_MAX_SIZE: usize = 4096;

pub struct GifImage {
    color_table: Rc<RefCell<Vec<Pixel>>>,
    /// Image of u8 which represents the indexes of the colors
    image: Image<u8>,
    left: u16,
    top: u16,
    height: u16,
    width: u16,
    delay: u16,
}

impl ImageFormat<Color> for GifImage {
    fn fill(&mut self, color: &Color) -> &mut Self {
        let color_index = self.get_color_index(color) as u8;

        self.image.fill(color_index);

        self
    }

    fn draw_rectangle(
        &mut self,
        top_left: Position,
        bottom_right: Position,
        color: &Color,
    ) -> &mut Self {
        let color_index = self.get_color_index(color) as u8;

        self.image
            .draw_rectangle(top_left, bottom_right, color_index);

        self
    }

    fn draw_text(
        &mut self,
        letter: &str,
        font_size: FontSize,
        top_left: crate::common::types::IPosition,
        color: &Color,
    ) -> &mut Self {
        let color_index = self.get_color_index(color) as u8;

        self.image
            .draw_text(letter, font_size, top_left, color_index);

        self
    }
}

impl GifImage {
    pub fn new(height: u16, width: u16, color_table: Option<Rc<RefCell<Vec<Pixel>>>>) -> Self {
        // TODO if no colorTable create local one
        let color_table = match color_table {
            Some(ct) => ct,
            None => Rc::new(RefCell::new(Vec::new())),
        };

        GifImage {
            color_table: color_table,
            // pixel_indexes: vec![0u8; height as usize * width as usize],
            image: Image::new(height as usize, width as usize, 0),
            left: 0,
            top: 0,
            height,
            width,
            delay: 0,
        }
    }

    /// add left beginning of image
    pub fn add_left(&mut self, position: u16) -> &mut GifImage {
        self.left = position;
        self
    }

    /// add top beginning of image
    pub fn add_top(&mut self, position: u16) -> &mut GifImage {
        self.top = position;
        self
    }

    /// add delay in centiseconds. Ex: delay = 10 = 0.1s
    pub fn add_delay(&mut self, delay: u16) -> &mut GifImage {
        self.delay = delay;
        self
    }

    fn get_color_index(&mut self, color: &Color) -> usize {
        let mut color_table = self.color_table.borrow_mut();

        let index = match &color_table.iter().position(|c| c == color) {
            Some(index) => *index,
            None => {
                color_table.push(color.clone());
                color_table.len() - 1
            }
        };

        index
    }

    /** Converting the image to bytes **/
    pub fn as_bytes(&self) -> Vec<u8> {
        let img_descriptor = self.get_image_descriptor();
        let img_data = self.get_image_data();

        let mut bytes = Vec::with_capacity(
            GRAPHICAL_CONTROL_EXTENSION_SIZE + IMAGE_DESCRIPTOR_SIZE + img_data.len(),
        );

        if self.delay != 0 {
            bytes.extend_from_slice(&self.get_gce());
        }

        bytes.extend_from_slice(&img_descriptor);
        bytes.extend_from_slice(&img_data);

        bytes
    }

    // Graphic Control Extension
    pub fn get_gce(&self) -> Vec<u8> {
        let mut bytes = vec![0u8; GRAPHICAL_CONTROL_EXTENSION_SIZE];

        bytes[0] = EXTENSIONS_INTRODUCER;
        bytes[1] = GRAPHIC_CONTROL_LABEL;
        bytes[2] = GCE_BYTE_SIZE;

        // TODO implement
        bytes[3] = 0; // Packed field

        // adding delay bytes
        let delay_bytes = self.delay.to_le_bytes();
        bytes[4] = delay_bytes[0];
        bytes[5] = delay_bytes[1];

        // NOTE remaining bytes not important for now

        // println!("GCE: {:02X?}", bytes);

        bytes
    }

    pub fn get_image_descriptor(&self) -> Vec<u8> {
        let mut img_desc = Vec::with_capacity(IMAGE_DESCRIPTOR_SIZE);

        img_desc.push(IMAGE_SEPARATOR);
        img_desc.extend(self.left.to_le_bytes());
        img_desc.extend(self.top.to_le_bytes());
        img_desc.extend(self.width.to_le_bytes());
        img_desc.extend(self.height.to_le_bytes());

        // FIXME NOTE for now packed field will be 0, but will probably change in the future
        img_desc.push(0); // packed field

        img_desc
    }

    pub fn get_image_data(&self) -> Vec<u8> {
        let (lzw_min_code_size, encoded) = self.encode_to_lzw();

        let size_of_encoded = encoded.len();
        let num_sub_block = size_of_encoded / SUB_BLOCK_MAX_SIZE;
        let num_byte_last_block = size_of_encoded % SUB_BLOCK_MAX_SIZE;

        let total_num_sub_block = if num_byte_last_block > 0 {
            num_sub_block + 1
        } else {
            num_sub_block
        };

        // lzw_min_code_size (1 byte) +
        // size_of_encoded bytes +
        // total_num_sub_block bytes (1 byte for each block) +
        // block terminator (1 byte)
        let mut img_data = Vec::with_capacity(2 + total_num_sub_block + size_of_encoded);
        img_data.push(lzw_min_code_size);

        // Adding full blocks
        for i in 0..num_sub_block {
            img_data.push(SUB_BLOCK_MAX_SIZE as u8); // Adding block size
            img_data.extend_from_slice(
                &encoded[(SUB_BLOCK_MAX_SIZE * i)..(i + 1) * SUB_BLOCK_MAX_SIZE],
            );
        }

        // Adding the last block
        if num_byte_last_block > 0 {
            img_data.push(num_byte_last_block as u8);
            img_data.extend_from_slice(&encoded[size_of_encoded - num_byte_last_block..]);
        }

        // Pushing the block terminator
        img_data.push(0);

        return img_data;
    }

    /// Encode input to lzw and variable length code
    // fn encode_to_lzw(input: &[u8], num_unique_code: usize) -> (u8, Vec<u8>) {
    fn encode_to_lzw(&self) -> (u8, Vec<u8>) {
        let num_unique_code = self.color_table.borrow().len();
        // Initializing code table
        let mut table: Vec<Vec<usize>> = Vec::with_capacity(CODE_TABLE_MAX_SIZE);
        for i in 0..num_unique_code {
            table.push(vec![i]);
        }

        // Adding CC (clear code) and EOI (End of Information) to the table
        let (cc, eoi) = (num_unique_code, num_unique_code + 1);
        table.push(vec![cc]); // Adding Clear Code
        table.push(vec![eoi]); // Adding End Of Information

        let mut lzw_min_code_size = 0u8;
        while 1 << (lzw_min_code_size + 1) < table.len() {
            lzw_min_code_size += 1;
        }

        let mut curr_code_size = lzw_min_code_size + 1;

        let mut encoded = BitCoder::new();
        encoded.write_code(cc, curr_code_size);

        let mut pixel_indexes = self.image.pixels_indexes.clone();

        // Taking first value for index_buffer
        let first = &mut pixel_indexes[0];
        let mut index_buffer = vec![first.value as usize];
        first.freq -= 1;

        // for rle in &self.image.pixels_indexes {
        for rle in pixel_indexes {
            for _ in 0..rle.freq {
                let k = rle.value as usize;

                let mut index_buffer_plus_k = index_buffer.clone();
                index_buffer_plus_k.push(k);

                let index = table.iter().position(|code| *code == index_buffer_plus_k);

                match index {
                    Some(_index) => {
                        index_buffer.push(k);
                    }
                    None => {
                        table.push(index_buffer_plus_k.clone());

                        // Writing current code to buffer
                        let code = table.iter().position(|code| *code == index_buffer).unwrap();

                        encoded.write_code(code, curr_code_size);

                        // Update the code size when the number of element in array
                        // surpasses the number of bits needed to write their index
                        if (1 << curr_code_size) < table.len() {
                            curr_code_size += 1;
                        } else if table.len() == CODE_TABLE_MAX_SIZE - 1 {
                            // Resetting the table when reaching max size
                            encoded.write_code(cc, curr_code_size);
                            table.truncate(eoi + 1); // Since eoi is the last index of the table
                            curr_code_size = lzw_min_code_size + 1;
                        }

                        // Set index buffer to k
                        index_buffer = vec![k];
                    }
                }
            }
        }
        // Adding last index value
        let last_index_value = table.iter().position(|code| *code == index_buffer).unwrap();
        encoded.write_code(last_index_value, curr_code_size);

        // Adding End of information code
        encoded.write_code(eoi, curr_code_size);

        // Writing the last byte in the bit coder
        encoded.flush();

        (lzw_min_code_size, encoded.buffer)
    }
}

struct BitCoder {
    buffer: Vec<u8>,
    curr_byte: u8,
    bit_pos: u8,
}

impl BitCoder {
    pub fn new() -> Self {
        BitCoder {
            buffer: Vec::new(),
            curr_byte: 0,
            bit_pos: 0,
        }
    }

    pub fn write_code(&mut self, code: usize, length: u8) {
        for i in 0..length {
            let bit = (code >> i) & 1;

            self.curr_byte |= (bit as u8) << self.bit_pos;
            self.bit_pos += 1;

            if self.bit_pos == 8 {
                self.flush();
            }
        }
    }

    pub fn flush(&mut self) {
        self.buffer.push(self.curr_byte);
        self.curr_byte = 0;
        self.bit_pos = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Based on the following example: https://giflib.sourceforge.net/whatsinagif/bits_and_bytes.html#image_data_block
    #[test]
    fn image_data_test() {
        let white = Pixel::new(255, 255, 255);
        let red = Pixel::new(255, 0, 0);
        let blue = Pixel::new(0, 0, 255);
        let black = Pixel::new(0, 0, 0);

        let color_table = Rc::new(RefCell::new(vec![
            white.clone(),
            red.clone(),
            blue.clone(),
            black.clone(),
        ]));

        let mut img = GifImage::new(10, 10, Some(Rc::clone(&color_table)));
        img.fill(&red)
            .draw_rectangle(Position::new(5, 0), Position::new(9, 4), &blue)
            .draw_rectangle(Position::new(0, 5), Position::new(4, 9), &blue)
            .draw_rectangle(Position::new(3, 3), Position::new(6, 6), &white);

        // DEBUG
        // img.image.print_content();

        // println!("color_table: {:?}", color_table);
        let result = img.get_image_data();
        let expected = vec![
            b'\x02', // lzw_min_code_size
            b'\x16', // Number of bytes in sub block
            // Data sub block
            b'\x8C', b'\x2D', b'\x99', b'\x87', b'\x2A', b'\x1C', b'\xDC', b'\x33', b'\xA0',
            b'\x02', b'\x75', b'\xEC', b'\x95', b'\xFA', b'\xA8', b'\xDE', b'\x60', b'\x8C',
            b'\x04', b'\x91', b'\x4C', b'\x01',
            // End of block
            b'\x00', // Block Terminator
        ];

        assert_eq!(expected, result);
    }
}
