use crate::new_renderer::pixel::Pixel;
use crate::new_renderer::ImmediateRenderer;
use font8x8::{UnicodeFonts, BASIC_FONTS, BLOCK_FONTS};

impl ImmediateRenderer {
    pub fn place_char(&mut self, x: usize, y: usize, c: char, p: Pixel) -> Option<()> {
        let fm = BASIC_FONTS.get(c);
        let f_mask = match fm {
            Some(x) => x,
            None => {
                let fm1 = BLOCK_FONTS.get(c);
                match fm1 {
                    Some(x) => x,
                    None => {
                        return None;
                    }
                }
            }
        };
        for (i, row) in f_mask.iter().enumerate() {
            for bit in 0..8 {
                match *row & 1 << bit {
                    0 => {}
                    _ => *self.get_pixel_mut(x + i, y + bit as usize)? = p,
                }
            }
        }
        Some(())
    }

    pub fn place_string(&mut self, x: usize, y: usize, s: &str, color: Pixel) -> Option<()> {
        for (i, c) in s.chars().enumerate() {
            self.place_char(x, y + i * 9, c, color)?
        }
        Some(())
    }
}
