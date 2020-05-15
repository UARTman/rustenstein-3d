use crate::renderer::Renderer;
use font8x8::{BASIC_FONTS, UnicodeFonts, LATIN_FONTS, MISC_FONTS, SGA_FONTS};
use crate::renderer::pixel::Pixel;

impl Renderer {
    pub fn place_char(&mut self, x: usize, y: usize, c: char, color: Pixel) -> Option<()> {
        let f_mask = BASIC_FONTS.get(c)?;
        for (i, row) in f_mask.iter().enumerate() {
            for bit in 0..8 {
                match *row & 1 << bit {
                    0 => {}
                    _ => { *self.get_mut(x + i, y + bit as usize)? = color }
                }
            }
        }
        Some(())
    }

    pub fn place_string(&mut self, x: usize, y: usize, s: &str, color: Pixel) -> Option<()> {
        for (i, c) in s.chars().enumerate() {
            self.place_char(x , y+ i * 9, c, color)?
        }
        Some(())
    }
}