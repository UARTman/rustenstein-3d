use crate::new_renderer::shader::Shader;
use crate::new_renderer::pixel::{Pixel, rgb};
use std::cmp::{max, min};

pub struct OldWallShader {
    pub draw_limit: f32
}

impl Shader for OldWallShader {
    fn sample(&self, distance: f32, sx: f32, sy: f32) -> Pixel {
        if ((sy * 100.0) as usize) < 1 || (((1.0 - sy) * 100.0) as usize) < 1 {
            return rgb(0, 0, 0);
        }
        let coefficient = 1.0 - distance / self.draw_limit;
        let grayscale = (255.0 * coefficient) as u32;
        rgb(grayscale, grayscale, grayscale)
    }
}

pub fn sample_wall(x: f32, y: f32) -> f32 {
    let fx = x - x.floor();
    let fy = y - y.floor();
    let ux = (fx * 10.0) as usize;
    let uy = (fy * 10.0) as usize;
    let nx = 10 - ux;
    let ny = 10 - uy;
    let m = min(min(ux, nx), min(uy, ny));
    if m == ux {
        return 1.0 - fy;
    }
    if m == uy {
        return fx;
    }
    if m == nx {
        return fy;
    }
    if m == ny {
        return 1.0 - fx;
    }

    panic!()
}