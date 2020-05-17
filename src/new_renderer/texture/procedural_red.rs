use crate::color::rgb;
use crate::new_renderer::texture::Texture;

pub struct ProceduralRedTexture {}

impl Texture for ProceduralRedTexture {
    fn sample_pixel(&self, x: f32, y: f32) -> u32 {
        rgb((y * 255.0) as u32, 0, (x * 255.0) as u32)
    }
}
