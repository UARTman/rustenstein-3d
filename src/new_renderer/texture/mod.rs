use crate::new_renderer::pixel::Pixel;

pub mod color;
pub mod simple_sprite;

pub trait Texture {
    fn sample_pixel(&self, x: f32, y: f32) -> Pixel;
}
