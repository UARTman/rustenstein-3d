use crate::new_renderer::shader::Shader;
use crate::new_renderer::texture::Texture;

pub struct UnlitTextureShader {
    pub texture: Box<dyn Texture>,
}

impl Shader for UnlitTextureShader {
    fn sample(&self, _: f32, sx: f32, sy: f32) -> u32 {
        self.texture.sample_pixel(sx, sy)
    }
}
