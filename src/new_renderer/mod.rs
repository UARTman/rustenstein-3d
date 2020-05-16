use minifb::{Window, Error};
use crate::new_renderer::pixel::Pixel;

pub mod pixel;

pub struct ImmediateRenderer {
    window: Window,
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl ImmediateRenderer {
    fn new(window: Window) -> Self {
        let (width, height) = window.get_size();
        Self {
            window,
            buffer: vec![0; width * height],
            width,
            height,
        }
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.window.update_with_buffer(&self.buffer, self.width, self.height)
    }

    fn pos(&self, x: usize, y: usize) -> usize {
        x * self.width + y
    }

    fn get_pixel(&self, x: usize, y: usize) -> Option<&Pixel> {
        self.buffer.get(self.pos(x, y))
    }

    fn get_pixel_mut(&mut self, x: usize, y: usize) -> Option<&mut Pixel> {
        self.buffer.get_mut(self.pos(x, y))
    }
}