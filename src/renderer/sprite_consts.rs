use crate::renderer::pixel_colors::*;
use crate::renderer::sprite::Sprite8;

pub const MINIMAP_WALL: Sprite8 = Sprite8 {
    pixels: [[WHITE; 8]; 8],
};

pub const MINIMAP_PLAYER: Sprite8 = Sprite8 {
    pixels: [
        [BLACK, BLACK, BLACK, GREEN, GREEN, BLACK, BLACK, BLACK],
        [BLACK, BLACK, BLACK, GREEN, GREEN, BLACK, BLACK, BLACK],
        [BLACK, BLACK, BLACK, GREEN, GREEN, BLACK, BLACK, BLACK],
        [GREEN, GREEN, GREEN, GREEN, GREEN, GREEN, GREEN, GREEN],
        [GREEN, GREEN, GREEN, GREEN, GREEN, GREEN, GREEN, GREEN],
        [BLACK, BLACK, BLACK, GREEN, GREEN, BLACK, BLACK, BLACK],
        [BLACK, BLACK, BLACK, GREEN, GREEN, BLACK, BLACK, BLACK],
        [BLACK, BLACK, BLACK, GREEN, GREEN, BLACK, BLACK, BLACK],
    ],
};
