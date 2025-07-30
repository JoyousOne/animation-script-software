use super::types::Color;

pub struct Scene {
    objects: Vec<Box<dyn Draw>>,
    xsize: usize,
    ysize: usize,
    frame_count: u32,
}

impl Scene {
    pub fn new(xsize: usize, ysize: usize, frame_count: u32) -> Self {
        Self {
            objects: vec![],
            xsize,
            ysize,
            frame_count,
        }
    }

    pub fn add_object(&mut self, object: impl Draw + 'static) {
        self.objects.push(Box::from(object));
    }

    pub fn render(mut self) -> Vec<Frame> {
        let mut frames = vec![];

        self.objects
            .sort_by(|obj1, obj2| obj1.zindex().cmp(&obj2.zindex()));

        for frame_counter in 0..self.frame_count {
            let mut frame = Frame::new(self.xsize, self.ysize);

            for object in &self.objects {
                object.draw(frame_counter, &mut frame);
            }

            frames.push(frame);
        }

        frames
    }
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub buffer: Vec<Vec<Color>>,
}

impl Frame {
    pub fn new(xsize: usize, ysize: usize) -> Self {
        Self {
            buffer: vec![vec![Color::RGBA(0, 0, 0, 0); xsize]; ysize],
        }
    }

    pub fn xsize(&self) -> usize {
        self.buffer.get(0).unwrap_or(&vec![]).len()
    }

    pub fn ysize(&self) -> usize {
        self.buffer.len()
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, c: Color) {
        if let Some(row) = self.buffer.get_mut(y) {
            if let Some(value) = row.get_mut(x) {
                *value = c;
            }
        }
    }
}

pub trait Draw {
    fn zindex(&self) -> i32;

    fn draw(&self, frame_count: u32, frame: &mut Frame);
}
