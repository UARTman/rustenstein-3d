use crate::new_renderer::pixel::Pixel;

pub mod old_wall;
pub mod texture;
pub mod lit_texture;

pub trait Shader {
    fn sample(&self, distance: f32, sx: f32, sy: f32) -> Pixel;
}