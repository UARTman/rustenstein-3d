use crate::renderer::pixel::Pixel;

pub const WHITE: Pixel = Pixel {
    r: 255,
    g: 255,
    b: 255,
};

pub const BLACK: Pixel = Pixel { r: 0, g: 0, b: 0 };

pub const RED: Pixel = Pixel { r: 255, g: 0, b: 0 };

pub const GREEN: Pixel = Pixel { r: 0, g: 255, b: 0 };

pub const BLUE: Pixel = Pixel { r: 0, g: 0, b: 255 };
