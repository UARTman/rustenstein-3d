use crate::renderer::pixel::Pixel;
use crate::renderer::Renderer;

pub struct Sprite8 {
    pub pixels: [[Pixel; 8]; 8],
}

impl Sprite8 {
    pub fn new(pixels: [[Pixel; 8]; 8]) -> Self {
        Self { pixels }
    }

    pub fn render(&self, x: usize, y: usize, renderer: &mut Renderer) -> Option<()> {
        for i in 0..8 {
            for j in 0..8 {
                *renderer.get_mut(x + i, y + j)? = self.pixels[i][j]
            }
        }
        Some(())
    }
}
