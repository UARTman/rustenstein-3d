use crate::new_renderer::pixel::shade;
use crate::new_renderer::shader::Shader;
use crate::new_renderer::texture::Texture;

pub struct LitTextureShader {
    pub draw_limit: f32,
    pub texture: Box<dyn Texture>,
}

impl Shader for LitTextureShader {
    fn sample(&self, distance: f32, sx: f32, sy: f32) -> u32 {
        let coefficient = 1.0 - distance / self.draw_limit;
        shade(self.texture.sample_pixel(sx, sy), coefficient)
    }
}
