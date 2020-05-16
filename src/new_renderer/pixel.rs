pub type Pixel = u32;

pub fn rgb(r: u32, g: u32, b: u32) -> Pixel {
    (((r << 8) + g) << 8) + b
}

pub fn grayscale(s: u32) -> Pixel {
    rgb(s, s, s)
}
