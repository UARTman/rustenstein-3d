use crate::new_renderer::pixel::Pixel;
use winit::{window::Window, dpi::{LogicalSize, PhysicalSize}};

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
        let  PhysicalSize { width, height}: PhysicalSize<u32> = window.inner_size();
        Self {
            buffer: vec![0; (width * height) as usize],
            width: width as usize,
            height: height as usize,
        }
    }

    pub fn render_to(&self, buf: &mut [u8]) {
        for (i, &p) in self.buffer.iter().enumerate() {
            buf[4 * i] = (p & 0xFF) as u8;
            buf[4 * i + 1] = ((p >> 8) & 0xFF) as u8;
            buf[4 * i + 2] = ((p >> 16) & 0xFF) as u8;
            buf[4 * i + 3] = 0xFF;
        }
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
