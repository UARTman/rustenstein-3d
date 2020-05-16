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
    pub fn move_player(&mut self, x: f32, y: f32) {
        self.player.go(x, y);
        if self.collide_player() {
            self.player.go(-x, -y);
        }
    }

    fn collide_player(&self) -> bool {
        self.field.field[self.player.x as usize][self.player.y as usize] != '#'
    }

    pub fn render_map(&self, renderer: &mut ImmediateRenderer) {
        for (i, row) in self.field.field.iter().enumerate() {
            for (j, px) in row.iter().enumerate() {
                renderer.place_char(24 + i * 8, j * 8, *px, rgb(0, 255, 255));
            }
        }
        renderer.place_char(24 + (self.player.x * 8.0) as usize, (self.player.y * 8.0) as usize, '@', rgb(255, 0, 255));
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