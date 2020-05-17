use crate::new_renderer::pixel::Pixel;

pub mod procedural_red;
pub mod procedural_grass;
pub mod simple_sprite;
pub mod color;

pub trait Texture {
    fn sample_pixel(&self, x: f32, y: f32) -> Pixel;
}