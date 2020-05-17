use crate::new_renderer::pixel::Pixel;
use crate::new_renderer::texture::Texture;

pub struct ColorTexture {
    pub color: Pixel,
}

impl Texture for ColorTexture {
    fn sample_pixel(&self, _x: f32, _y: f32) -> u32 {
        self.color
    }
}
