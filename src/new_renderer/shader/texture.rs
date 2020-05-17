use crate::new_renderer::texture::Texture;
use crate::new_renderer::shader::Shader;

pub struct TextureShader {
    pub texture: Box<dyn Texture>
}

impl Shader for TextureShader {
    fn sample(&self, _: f32, sx: f32, sy: f32) -> u32 {
        self.texture.sample_pixel(sx, sy)
    }
}