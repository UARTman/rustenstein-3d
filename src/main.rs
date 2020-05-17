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

const WIDTH: usize = 600;
const HEIGHT: usize = 300;

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


    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut renderer = ImmediateRenderer::new(&window);

    let mut timer = Timer::new();

    let mut game = Game::default();


    while window.is_open() && !window.is_key_down(Key::Escape) {
        timer.tick();
        let tdt = timer.dt.as_secs_f32();

        let (mut gf, mut gr) : (f32, f32)= (0.0, 0.0);
        if window.is_key_down(Key::W) {
            gf = 1.0 * tdt
        }
        if window.is_key_down(Key::S) {
            gf = -1.0 * tdt
        }
        if window.is_key_down(Key::A) {
            gr = -1.0 * tdt
        }
        if window.is_key_down(Key::D) {
            gr = 1.0 * tdt
        }
        if gf != 0.0 || gr != 0.0 {
            game.move_player(gf, gr)
        }

        if window.is_key_down(Key::Q) {
            game.player.rotate(0.4 * tdt)
        }
        if window.is_key_down(Key::E) {
            game.player.rotate(-0.4 * tdt)
        }

        renderer.clear();
        game.render_walls(&mut renderer, 2.8, WIDTH, HEIGHT);
        game.render_map(&mut renderer);
        renderer.place_string(0, 0, format!("FPS: {}", 1.0 / tdt).as_str(), rgb(255, 255, 0));
        renderer.place_string(8, 0, format!("Player angle: {}", game.player.angle).as_str(), rgb(255, 255, 0));

        renderer.flush(&mut window).unwrap();
    }
}
