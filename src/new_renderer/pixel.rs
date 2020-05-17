pub type Pixel = u32;

pub fn rgb(r: u32, g: u32, b: u32) -> Pixel {
    (((r << 8) + g) << 8) + b
}

pub fn grayscale(s: u32) -> Pixel {
    rgb(s, s, s)
}

pub fn to_rgb(mut px: Pixel) -> (u32, u32, u32) {
    let blue = px % (1 << 8);
    px /= 1 << 8;
    let green = px % (1 << 8);
    px /= 1 << 8;
    let red = px % (1 << 8);
    (red, green, blue)
}

pub fn shade(pixel: Pixel, coefficient: f32) -> Pixel {
    let (red, green, blue) = to_rgb(pixel);
    rgb(
        (red as f32 * coefficient) as u32,
        (green as f32 * coefficient) as u32,
        (blue as f32 * coefficient) as u32,
    )
}

#[cfg(test)]
mod tests {
    use crate::new_renderer::pixel::{rgb, to_rgb};

    #[test]
    fn test_unpixel() {
        assert_eq!(to_rgb(rgb(1, 2, 3)), (1, 2, 3));
    }
}
