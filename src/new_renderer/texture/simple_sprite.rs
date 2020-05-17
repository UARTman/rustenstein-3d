use crate::new_renderer::pixel::Pixel;
use crate::new_renderer::texture::Texture;

pub struct SimpleSpriteTexture {
    pub sprite: Vec<Vec<Pixel>>,
    pub width: usize,
    pub height: usize
}

impl Texture for SimpleSpriteTexture {
    fn sample_pixel(&self, x: f32, y: f32) -> u32 {
        self.sprite[((x - x.floor()).abs() * self.height as f32) as usize][((y - y.floor()).abs() * self.width as f32) as usize]
    }
}