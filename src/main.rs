extern crate minifb;

use minifb::{Key, Window, WindowOptions};

use crate::renderer::pixel_colors::RED;
use crate::renderer::sprite_consts::{MINIMAP_PLAYER, MINIMAP_WALL};
use crate::timer::Timer;

pub mod color;
pub mod renderer;
pub mod timer;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut px_renderer = renderer::Renderer::new(WIDTH, HEIGHT);

    let mut timer = Timer::new();

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let (mut px, mut py) = (50.0, 50.0);


    while window.is_open() && !window.is_key_down(Key::Escape) {
        timer.tick();
        px_renderer.clear();
        py = (py + 20.0 * timer.dt.as_secs_f32()) % WIDTH as f32;
        MINIMAP_PLAYER.render(px as usize, py as usize, &mut px_renderer);
        px_renderer.render(&mut buffer);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}