extern crate minifb;

use minifb::{Key, Window, WindowOptions};

use crate::timer::Timer;
use crate::new_renderer::ImmediateRenderer;
use crate::new_renderer::pixel::rgb;
use crate::game::Game;

pub mod color;
pub mod timer;
pub mod new_renderer;
pub mod game;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });


    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut renderer = ImmediateRenderer::new(&window);

    let mut timer = Timer::new();

    let mut game = Game::default();


    while window.is_open() && !window.is_key_down(Key::Escape) {
        timer.tick();

        renderer.clear();
        game.render_map(&mut renderer);
        renderer.place_string(0, 0, format!("DT: {}", timer.dt.as_secs_f32()).as_str(), rgb(255, 255, 0));
        renderer.place_string(8, 0, format!("FPS: {}", 1.0 / timer.dt.as_secs_f32()).as_str(), rgb(255, 255, 0));

        renderer.flush(&mut window).unwrap();
    }
}
