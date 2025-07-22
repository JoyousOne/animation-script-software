use crate::common::types::{Color, Pixel};
use crate::format::gif::gif_image::Image;
use std::{cell::RefCell, cmp::min, fs::File, io::Write, rc::Rc};

pub const SIGNATURE: [u8; 3] = *b"GIF";
pub const VERSION: [u8; 3] = *b"89a";
pub const TRAILER_MARKER: u8 = 0x3B;

pub const APPLICATION_EXTENSION_BLOCK: [u8; 19] = [
    0x21, // Extension introducer
    0xFF, // Application extension label
    0x0B, // NETSCAPE size
    b'N', b'E', b'T', b'S', b'C', b'A', b'P', b'E', b'2', b'.', b'0', // NETSCAPE2.0
    0x03, // Application extension sub-block length
    0x01, // Sub-block ID (looping)
    0x00, 0x00, // Loop count (can be replaced)
    0x00, // Block terminator
];
pub const LOOP_COUNT_STARTING_INDEX: usize = 16;

pub enum Loop {
    Forever,
    Repeat(u16),
}

pub struct Gif {
    height: u16,
    width: u16,
    global_color_table: Rc<RefCell<Vec<Color>>>,
    num_loop: Option<Loop>,
    images: Vec<Image>, // TODO add image type
}

impl Gif {
    pub fn new(height: u16, width: u16, num_loop: Option<Loop>) -> Self {
        Gif {
            height,
            width,
            global_color_table: Rc::new(RefCell::new(Vec::new())),
            images: Vec::new(),
            num_loop,
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


    fn get_application_extension_block(&self) -> Vec<u8> {
        if let Some(num_loop) = &self.num_loop {
            let mut aeb: Vec<u8> = APPLICATION_EXTENSION_BLOCK.to_vec();

            // Adjust num of loop if specified
            if let Loop::Repeat(n) = num_loop {
                let num_loop = n.to_le_bytes();

                let mut i = LOOP_COUNT_STARTING_INDEX;
                for byte in num_loop {
                    aeb[i] = byte;
                    i += 1;
                }
            }

            aeb
        } else {
            Vec::with_capacity(0)
        }
    }

    pub fn write_to_file(&mut self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut file_contents = Vec::new();
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
            println!("global_color_bits: {}", global_color_bits);

            // Filling global color table with padding
            let padding_size = (1 << (global_color_bits + 1)) - global_color_table.len();
            println!("padding_size: {}", padding_size);
            for _ in 0..padding_size {
                global_color_table.push(Color::new(0, 0, 0));
            }

            // add colors
            for color in global_color_table.iter() {
                file_contents.extend(color.as_bytes());
            }
        }

        println!("color: {:?}", self.global_color_table);

        // Adding application extension if present
        file_contents.extend_from_slice(&self.get_application_extension_block());

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

    
    #[rustfmt::skip]
    pub fn debug(&self, num_byte_per_row: Option<usize>) {
        let mut to_print = Vec::new();
        const RED: &'static str= "\x1B[31m";
        const GREEN: &'static str= "\x1B[32m";
        const YELLOW: &'static str= "\x1B[33m";
        const BLUE: &'static str= "\x1B[34m";
        const DARK_MAGENTA: &'static str= "\x1B[95m";
        const DARK_CYAN: &'static str= "\x1B[96m";
        const RESET: &'static str= "\x1B[0m";

        // Printing legend
        println!("Legend: ");
        println!("  {YELLOW}Header Block{RESET}");
        println!("  {BLUE}Logistical Screen Descriptor{RESET}");
        println!("  Global Color Table: colors identified by their background colors");
        println!("  {GREEN}Application Extension Block{RESET}");
        println!(
            "  Images:\
            \n    - {RED}Graphical Control extension{RESET}\
            \n    - {BLUE}Image Descriptor{RESET}\
            \n    - {DARK_MAGENTA}Image Data{RESET}"
        );
        println!("  {DARK_CYAN}Trailer Marker{RESET}");
        println!();

        // add header block
        let header_block = vec![SIGNATURE, VERSION].concat();
        for byte in header_block {
            to_print.push(format!("{YELLOW}{:02X?}{RESET}", byte));
        }

        // add logistical screen descriptor
        let mut logistical_screen_descriptor =
            vec![self.width.to_le_bytes(), self.height.to_le_bytes()].concat();

        // add packed field
        let global_color_bits = self.get_global_color_table_size_bits();
        let packed_field = 0x90 | global_color_bits;
        logistical_screen_descriptor.push(packed_field);

        // add background color index
        logistical_screen_descriptor.push(0);

        // add pixel aspect ratio
        logistical_screen_descriptor.push(0);

        for byte in logistical_screen_descriptor {
            to_print.push(format!("{BLUE}{:02X}{RESET}", byte));
        }

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
                let color_bytes = color.as_bytes();
                let Pixel { r, g, b } = *color;

                let custom_bg = format!("\x1B[48;2;{r};{g};{b}m");
                for byte in color_bytes {
                    to_print.push(format!("{custom_bg}{:02X}{RESET}", byte));
                }
            }
        }

        // Adding application extension if present
        let application_extension_block = self.get_application_extension_block();
        for byte in application_extension_block {
            to_print.push(format!("{GREEN}{:02X?}{RESET}", byte));
        }

        // // add images
        for image in &self.images {
            // Printing Graphical Control Extension
            let gce_bytes = image.get_gce();
            if gce_bytes.len() > 0 {
                for byte in gce_bytes {
                    to_print.push(format!("{RED}{:02X?}{RESET}", byte));
                }
            }

            // Printing Image Descriptor
            let img_descriptor_bytes = image.get_image_descriptor();
            for byte in img_descriptor_bytes {
                to_print.push(format!("{BLUE}{:02X?}{RESET}", byte));
            }

            // Printing Image
            let img_data = image.get_image_data();
            for byte in img_data {
                to_print.push(format!("{DARK_MAGENTA}{:02X?}{RESET}", byte));
            }

            print!("{RESET}");
        }

        // Printing Trailer Mark
        to_print.push(format!("{DARK_CYAN}{TRAILER_MARKER}{RESET}"));

        print!("{}", to_print[0]);

        let num_byte_per_row= match num_byte_per_row {
            Some(num) => num,
            None => 16,
        };
        for i in 1..to_print.len() {
            print!("{}", to_print[i]);
            if (i + 1) % 2  == 0 { print!(" ") }
            if (i + 1) % num_byte_per_row == 0 { println!() }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use crate::common::types::Pixel;

    use super::*;
    #[test]
    fn gif_global_color_bits_test() {
        let mut gif = Gif::new(10, 10, None);

        let white = Pixel::new(255, 255, 255);
        let red = Pixel::new(255, 0, 0);
        let blue = Pixel::new(0, 0, 255);
        let black = Pixel::new(0, 0, 255);

        gif.add_image().fill(&white);
        assert_eq!(gif.get_global_color_table_size_bits(), 0);

        gif.add_image().fill(&red);
        assert_eq!(gif.get_global_color_table_size_bits(), 0);

        gif.add_image().fill(&blue);
        assert_eq!(gif.get_global_color_table_size_bits(), 1);

        gif.add_image().fill(&black);
        assert_eq!(gif.get_global_color_table_size_bits(), 1);
    }
}
