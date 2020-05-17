use crate::game::field::GameField;
use crate::game::player::Player;
use crate::new_renderer::ImmediateRenderer;
use crate::new_renderer::pixel::rgb;
use crate::new_renderer::shader::old_wall::{OldWallShader, sample_wall};
use crate::new_renderer::shader::Shader;
use crate::new_renderer::shader::lit_texture::LitTextureShader;
use crate::new_renderer::texture::procedural_red::ProceduralRedTexture;

pub mod field;
pub mod player;

pub struct Game {
    pub field: GameField,
    pub player: Player,
    pub wall_shader: Box<dyn Shader>,
}

impl Game {
    pub fn move_player(&mut self, forward: f32, right: f32) {
        self.player.go_forward(forward);
        self.player.go_right(right);
        if self.collide_player() {
            self.player.go_forward(-forward);
            self.player.go_right(-right);
        }
    }

    fn collide_player(&self) -> bool {
        self.field.field[self.player.x as usize][self.player.y as usize] == '#'
    }

    pub fn render_walls(&self, renderer: &mut ImmediateRenderer, fov: f32, width: usize, height: usize) {
        let hfov = fov / 2.0;
        let lbound = self.player.angle + hfov;
        let rbound = self.player.angle - hfov;
        let step = fov / width as f32;
        let mut angle = lbound;

        for px in 0..width {
            let raycast = self.raycast(self.player.x, self.player.y, angle, 0.01, 16.0);

            match raycast {
                Some((cx, cy, ray)) => {
                    let mut ho = (height as f32 / ray) as usize;
                    if ho > height / 2 {  // KOSTYL: Check for some float shenanigans.
                        ho = height / 2;
                    }

                    let offset = ((height / 2) - ho);


                    let ceil = offset;

                    let floor = height - offset;

                    let all = (height - 2 * offset);

                    for i in ceil..floor {
                        *renderer.get_pixel_mut(i, px).unwrap() = self.wall_shader.sample(ray, ((i - ceil) as f32 / all as f32), sample_wall(cx, cy, 1000.0));
                    }
                }
                None => {}
            }


            angle -= step;
        }
    }

    pub fn render_map(&self, renderer: &mut ImmediateRenderer) {
        for (i, row) in self.field.field.iter().enumerate() {
            for (j, px) in row.iter().enumerate() {
                renderer.place_char(24 + i * 8, j * 8, *px, rgb(0, 255, 255));
            }
        }
        renderer.place_char(24 + (self.player.x * 8.0) as usize, (self.player.y * 8.0) as usize, '@', rgb(255, 0, 255));
        let x = self.raycast(self.player.x, self.player.y, self.player.angle, 0.01, 100.0);
        match x {
            Some((vx, vy, _)) => { renderer.place_char(24 + vx as usize * 8, vy as usize * 8, '█', rgb(255, 0, 0)); }
            None => {}
        }
    }

    pub fn raycast(&self, mut x: f32, mut y: f32, angle: f32, step: f32, limit: f32) -> Option<(f32, f32, f32)> {
        let mut long = 0.0;

        loop {
            x += step * -angle.sin();
            y += step * angle.cos();
            if *self.field.field.get(x as usize)?.get(y as usize)? == '#' {
                return Some((x, y, long + step));
            }
            long += step;
            if long as usize >= limit as usize { //TODO: Remove kostyl
                return None;
            }
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            field: Default::default(),
            player: Default::default(),
            wall_shader: Box::new(
                LitTextureShader{
                    draw_limit: 16.0,
                    texture: Box::new(ProceduralRedTexture{})
                }
            )
        }
    }
}