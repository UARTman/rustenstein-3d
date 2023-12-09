use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use crate::{color::rgb, game::Game, new_renderer::ImmediateRenderer, timer::Timer};

const WIDTH: u32 = 600;
const HEIGHT: u32 = 380;

mod color;
mod timer;
mod game;
mod new_renderer;

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Rustenstein 3D")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };

    let mut renderer = ImmediateRenderer::new(&window);

    let mut timer = Timer::new();

    let mut game = Game::default();

    let char_speed = 2.0;
    let fov = f32::to_radians(75.0);

    renderer.clear();
    game.render_scene(&mut renderer, fov, WIDTH as usize, HEIGHT as usize);
    game.render_map(&mut renderer);
    renderer.place_string(
        0,
        0,
        format!("FPS: {}", 0).as_str(),
        rgb(255, 255, 0),
    );
    renderer.place_string(
        8,
        0,
        format!("Player angle: {}", game.player.angle).as_str(),
        rgb(255, 255, 0),
    );
    renderer.render_to(pixels.frame_mut());


    event_loop.run(move |event, _, control_flow| {
        timer.tick();
        let tdt = timer.dt.as_secs_f32();
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            renderer.clear();
            game.render_scene(&mut renderer, fov, WIDTH as usize, HEIGHT as usize);
            game.render_map(&mut renderer);
            renderer.place_string(
                0,
                0,
                format!("FPS: {}", 1.0 / tdt).as_str(),
                rgb(255, 255, 0),
            );
            renderer.place_string(
                8,
                0,
                format!("Player angle: {}", game.player.angle).as_str(),
                rgb(255, 255, 0),
            );
            renderer.render_to(pixels.frame_mut());
            pixels.render().unwrap();
            // world.draw(pixels.frame_mut());
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            let (mut gf, mut gr): (f32, f32) = (0.0, 0.0);
            if input.key_held(VirtualKeyCode::W) {
                gf = char_speed * tdt * 10000.;
            }
            if input.key_held(VirtualKeyCode::S) {
                gf = -char_speed * tdt * 10000.;
            }
            if input.key_held(VirtualKeyCode::A) {
                gr = -char_speed * tdt * 10000.;
            }
            if input.key_held(VirtualKeyCode::D) {
                gr = char_speed * tdt * 10000.;
            }
            if gf != 0.0 || gr != 0.0 {
                game.move_player(gf, gr)
            }
            if input.key_held(VirtualKeyCode::Q) {
                game.player.rotate(0.6 * tdt * 10000.);
            }
            if input.key_held(VirtualKeyCode::E) {
                game.player.rotate(-0.6 * tdt * 10000.);
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    // log_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Update internal state and request a redraw
            // world.update();
            window.request_redraw();
        }
    });
}
