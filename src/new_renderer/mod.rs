use crate::new_renderer::pixel::Pixel;
use minifb::{Error, Window};

pub mod pixel;
pub mod shader;
pub mod text;
pub mod texture;

pub struct ImmediateRenderer {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl ImmediateRenderer {
    pub fn new(window: &Window) -> Self {
        let (width, height) = window.get_size();
        Self {
            buffer: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn flush(&self, window: &mut Window) -> Result<(), Error> {
        window.update_with_buffer(&self.buffer, self.width, self.height)
    }

    fn pos(&self, x: usize, y: usize) -> usize {
        x * self.width + y
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Option<&Pixel> {
        self.buffer.get(self.pos(x, y))
    }

    pub fn get_pixel_mut(&mut self, x: usize, y: usize) -> Option<&mut Pixel> {
        let pos = self.pos(x, y);
        self.buffer.get_mut(pos)
    }

    pub fn clear(&mut self) {
        for i in self.buffer.iter_mut() {
            *i = 0;
        }
    }
}
