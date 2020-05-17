use crate::new_renderer::pixel::Pixel;
use crate::new_renderer::texture::Texture;

pub struct ColorTexture {
    pub color: Pixel
}

impl Texture for ColorTexture {
    fn sample_pixel(&self, x: f32, y: f32) -> u32 {
        self.color
    }
}