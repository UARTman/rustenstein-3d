pub type Pixel = u32;

pub fn rgb(r: u32, g: u32, b: u32) -> Pixel {
    (((r << 8) + g) << 8) + b
}

pub fn grayscale(s: u32) -> Pixel {
    rgb(s, s, s)
}

pub fn to_rgb(mut px: Pixel) -> (u32, u32, u32) {
    let b = px % (1 << 8);
    px = px / (1 << 8);
    let g = px % (1 << 8);
    px = px / (1 << 8);
    let r = px % (1 << 8);
    return (r, g, b);
}

pub fn shade(p: Pixel, c: f32 ) -> Pixel {
    let (r,g,b) = to_rgb(p);
    rgb((r as f32 * c) as u32, (g as f32 * c) as u32, (b as f32 * c) as u32 )
}

#[cfg(test)]
mod tests {
    use crate::new_renderer::pixel::{to_rgb, rgb};

    #[test]
    fn test_unpixel() {
        assert_eq!(to_rgb(rgb(1, 2, 3)), (1, 2, 3));
    }
}