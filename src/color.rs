pub fn rgb(r: u32, g: u32, b: u32) -> u32 {
    (((r << 8) + g) << 8) + b
}

pub fn grayscale(s: u32) -> u32 {
    rgb(s, s, s)
}
