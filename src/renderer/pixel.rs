#[derive(Copy, Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    pub fn color(self) -> u32 {
        crate::color::rgb(self.r as u32, self.g as u32, self.b as u32)
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn grayscale(s: u8) -> Self {
        Self { r: s, g: s, b: s }
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
}
