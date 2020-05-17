use crate::new_renderer::pixel::{rgb, Pixel};
use crate::new_renderer::shader::Shader;

pub struct OldWallShader {
    pub draw_limit: f32,
}

impl Shader for OldWallShader {
    fn sample(&self, distance: f32, _sx: f32, sy: f32) -> Pixel {
        if ((sy * 100.0) as usize) < 1 || (((1.0 - sy) * 100.0) as usize) < 1 {
            return rgb(0, 0, 0);
        }
        let coefficient = 1.0 - distance / self.draw_limit;
        let grayscale = (255.0 * coefficient) as u32;
        rgb(grayscale, grayscale, grayscale)
    }
}

