use crate::new_renderer::texture::Texture;
use crate::new_renderer::pixel::rgb;

pub struct ProceduralGrassTexture {}

impl Texture for ProceduralGrassTexture {
    fn sample_pixel(&self, x: f32, y: f32) -> u32 {
        rgb(0, (256.0 * (x - x.floor()).abs()) as u32, (256.0 * (y - y.floor()).abs()) as u32)
    }
}