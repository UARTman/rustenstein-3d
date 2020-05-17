use crate::new_renderer::pixel::Pixel;

pub mod color;
pub mod procedural_grass;
pub mod procedural_red;
pub mod simple_sprite;

pub trait Texture {
    fn sample_pixel(&self, x: f32, y: f32) -> Pixel;
}
