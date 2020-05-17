use crate::new_renderer::pixel::Pixel;

pub mod old_wall;

pub trait Shader {
    fn sample(&self, distance: f32, sx: f32, sy: f32) -> Pixel;
}