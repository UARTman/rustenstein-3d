use crate::game::field::GameField;
use crate::game::player::Player;
use crate::new_renderer::pixel::rgb;
use crate::new_renderer::shader::lit_texture::LitTextureShader;
use crate::new_renderer::shader::Shader;
use crate::new_renderer::texture::color::ColorTexture;
use crate::new_renderer::texture::simple_sprite::SimpleSpriteTexture;
use crate::new_renderer::ImmediateRenderer;
use std::cmp::min;

pub mod field;
pub mod player;

pub struct Game {
    pub field: GameField,
    pub player: Player,
    pub wall_shader: Box<dyn Shader>,
    pub floor_shader: Box<dyn Shader>,
    pub ceil_shader: Box<dyn Shader>,
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

    fn render_floor_and_ceiing(
        &self,
        renderer: &mut ImmediateRenderer,
        px: usize,
        height: usize,
        width: usize,
        ceiling: usize,
        floor: usize,
    ) {
        let sy = px as f32 / width as f32;
        let half_height = height / 2;
        for i in 0..ceiling {
            // Render ceiling
            *renderer.get_pixel_mut(i, px).unwrap() = self.ceil_shader.sample(
                i as f32 / half_height as f32,
                i as f32 / half_height as f32,
                sy,
            );
        }
        for i in floor..height {
            // Render floor
            let ah = height - i;
            *renderer.get_pixel_mut(i, px).unwrap() = self.floor_shader.sample(
                (ah as f32) / half_height as f32,
                1.0 - ((ah as f32) / half_height as f32),
                sy,
            );
        }
    }

    pub fn render_scene(
        &self,
        renderer: &mut ImmediateRenderer,
        fov: f32,
        width: usize,
        height: usize,
    ) {
        let half_fov = fov / 2.0;
        let lbound = self.player.angle + half_fov;
        // let rbound = self.player.angle - half_fov;
        let step = fov / width as f32;
        let mut angle = lbound;
        let half_height = height / 2;

        for px in 0..width {
            let result_of_raycast = self.raycast(self.player.x, self.player.y, angle, 0.01, 16.0); // Raycast for a column

            if let Some(raycast) = result_of_raycast {
                self.render_walls(renderer, width, height,  angle, px, raycast);
            } else {
                self.render_floor_and_ceiing(renderer, px, height, width, half_height, half_height);
            }

            angle -= step;
        }
    }

    fn render_walls(
        &self,
        renderer: &mut ImmediateRenderer,
        width: usize,
        height: usize,
        angle: f32,
        px: usize,
        (cx, cy, ray): (f32, f32, f32)
    ) {
        let real_distance = ray * (angle - self.player.angle).abs().cos(); // Fix fish-eye effect by calculating real distance

        let half_height = height / 2;
        let original_half_wall_height = (height as f32 / real_distance) as usize; // Find the half-height of a wall on a screen
        let mut half_wall_height = original_half_wall_height; // create a copy for rendering on screen, original remains for sampling texture
        if half_wall_height > half_height {
            // Set height in bounds of screen
            half_wall_height = half_height;
        }

        let offset = half_height - half_wall_height; // Find offset to wall

        let ceiling = offset; // Ceiling start coordinate

        let floor = height - offset; // Floor start coordinate

        let wall_sample = sample_wall(cx, cy, 1000.0); // Y position on a wall texture

        self.render_floor_and_ceiing(renderer, px, height, width, ceiling, floor);

        for i in ceiling..floor {
            // Render walls
            let sx = (i - ceiling + (original_half_wall_height - half_wall_height)) as f32
                / (2 * original_half_wall_height) as f32; // Sample x texture coordinate by comparing point to original half height
            *renderer.get_pixel_mut(i, px).unwrap() = self.wall_shader.sample(ray, sx, wall_sample);
            // Render a sampled pixel
        }
    }

    pub fn render_map(&self, renderer: &mut ImmediateRenderer) {
        for (i, row) in self.field.field.iter().enumerate() {
            for (j, px) in row.iter().enumerate() {
                renderer.place_char(24 + i * 8, j * 8, *px, rgb(0, 255, 255));
            }
        }
        renderer.place_char(
            24 + (self.player.x * 8.0) as usize,
            (self.player.y * 8.0) as usize,
            '@',
            rgb(255, 0, 255),
        );

        if let Some((vx, vy, _)) =
            self.raycast(self.player.x, self.player.y, self.player.angle, 0.8, 100.0)
        {
            renderer.place_char(24 + vx as usize * 8, vy as usize * 8, '█', rgb(255, 0, 0));
        }
    }

    pub fn raycast(
        &self,
        mut x: f32,
        mut y: f32,
        angle: f32,
        step: f32,
        limit: f32,
    ) -> Option<(f32, f32, f32)> {
        let mut long = 0.0;

        loop {
            x += step * -angle.sin();
            y += step * angle.cos();
            if *self.field.field.get(x as usize)?.get(y as usize)? == '#' {
                return Some((x, y, long + step));
            }
            long += step;
            if long as usize >= limit as usize {
                //TODO: Remove kostyl
                return None;
            }
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        let red = rgb(255, 0, 0);
        let white = rgb(255, 255, 255);
        let brick_wall_texture = vec![
            vec![white, red, red, red, white, red, red, red],
            vec![white; 8],
            vec![red, white, red, red, red, white, red, red],
            vec![white; 8],
            vec![red, red, white, red, red, red, white, red],
            vec![white; 8],
            vec![white, red, red, red, white, red, red, red],
            vec![white; 8],
        ];

        Self {
            field: Default::default(),
            player: Default::default(),
            wall_shader: Box::new(LitTextureShader {
                draw_limit: 16.0,
                texture: Box::new(SimpleSpriteTexture {
                    sprite: brick_wall_texture,
                    width: 8,
                    height: 8,
                }),
            }),
            floor_shader: Box::new(LitTextureShader {
                draw_limit: 1.0,
                texture: Box::new(ColorTexture {
                    color: rgb(0, 255, 0),
                }),
            }),
            ceil_shader: Box::new(LitTextureShader {
                draw_limit: 1.0,
                texture: Box::new(ColorTexture {
                    color: rgb(40, 40, 255),
                }),
            }),
        }
    }
}

pub fn sample_wall(x: f32, y: f32, precision: f32) -> f32 {
    let fx = x - x.floor();
    let fy = y - y.floor();
    let ux = (fx * precision) as usize;
    let uy = (fy * precision) as usize;
    let nx = precision as usize - ux;
    let ny = precision as usize - uy;
    let m = min(min(ux, nx), min(uy, ny));
    if m == ux {
        return 1.0 - fy;
    }
    if m == uy {
        return fx;
    }
    if m == nx {
        return fy;
    }
    if m == ny {
        return 1.0 - fx;
    }

    panic!()
}
