use crate::game::field::GameField;
use crate::game::player::Player;
use crate::new_renderer::ImmediateRenderer;
use crate::new_renderer::pixel::rgb;

pub mod field;
pub mod player;

pub struct Game {
    pub field: GameField,
    pub player: Player,
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
            let (cx, cy, ray) = self.raycast(self.player.x, self.player.y, angle, 0.01, 16.0).unwrap();


            let mut ho = (height as f32 / ray) as usize; // KOSTYL: Check for some float shenanigans.
            if ho > 180 {
                ho = 180;
            }

            let offset = ((height/ 2) - ho);
            // let offset = (ray * 12.0).powf(0.9) as usize;
            let coeff = 1.0 - ray / 16.0;


            let ceil = offset;

            let floor = height - offset;


            let grayscale = (255.0 * coeff) as u32;

            for i in offset..floor {
                *renderer.get_pixel_mut(i, px).unwrap() = rgb(grayscale, grayscale, grayscale);
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
        let (vx, vy, _) = self.raycast(self.player.x, self.player.y, self.player.angle, 0.01, 16.0).unwrap();
        renderer.place_char(24 + vx as usize * 8, vy as usize * 8, '█', rgb(255, 0, 0));
    }

    pub fn raycast(&self, mut x: f32, mut y: f32, angle: f32, step: f32, limit: f32) -> Option<(f32, f32, f32)> {
        let mut long = 0.0;

        loop {
            x += step * -angle.sin();
            y += step * angle.cos();
            if self.field.field[x as usize][y as usize] == '#' {
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
        }
    }
}