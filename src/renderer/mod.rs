use crate::renderer::pixel::Pixel;

pub mod pixel;
pub mod pixel_colors;
pub mod sprite;
pub mod sprite_consts;
pub mod char;

pub struct Renderer {
    pixel_buffer: Vec<Vec<Pixel>>,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        let pixel_buffer = vec![vec![Pixel::default(); width].clone(); height];
        Self { pixel_buffer }
    }

    fn translate_pixel_position(&self, x: usize, y: usize) -> usize {
        x * self.width() + y
    }

    pub fn render(&self, buffer: &mut [u32]) -> Option<()> {
        for (i, v) in self.pixel_buffer.iter().enumerate() {
            for (j, p) in v.iter().enumerate() {
                *buffer.get_mut(self.translate_pixel_position(i, j))? = p.color()
            }
        }
        Some(())
    }

    pub fn clear(&mut self) {
        for i in self.pixel_buffer.iter_mut() {
            for j in i.iter_mut() {
                *j = Pixel::default()
            }
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Pixel> {
        self.pixel_buffer.get(x)?.get(y)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Pixel> {
        self.pixel_buffer.get_mut(x)?.get_mut(y)
    }

    pub fn width(&self) -> usize {
        self.pixel_buffer[0].len()
    }

    pub fn height(&self) -> usize {
        self.pixel_buffer.len()
    }
}
