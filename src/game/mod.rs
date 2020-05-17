use crate::game::field::GameField;
use crate::game::player::Player;
use crate::new_renderer::pixel::rgb;
use crate::new_renderer::shader::lit_texture::LitTextureShader;
use crate::new_renderer::shader::old_wall::{sample_wall, OldWallShader};
use crate::new_renderer::shader::Shader;
use crate::new_renderer::texture::color::ColorTexture;
use crate::new_renderer::texture::procedural_grass::ProceduralGrassTexture;
use crate::new_renderer::texture::procedural_red::ProceduralRedTexture;
use crate::new_renderer::texture::simple_sprite::SimpleSpriteTexture;
use crate::new_renderer::ImmediateRenderer;

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

    pub fn render_walls(
        &self,
        renderer: &mut ImmediateRenderer,
        fov: f32,
        width: usize,
        height: usize,
    ) {
        let hfov = fov / 2.0;
        let lbound = self.player.angle + hfov;
        let rbound = self.player.angle - hfov;
        let step = fov / width as f32;
        let mut angle = lbound;
        let half_height = height / 2;

        for px in 0..width {
            let raycast = self.raycast(self.player.x, self.player.y, angle, 0.01, 16.0);

            if let Some((cx, cy, ray)) = raycast {
                let rd = ray * (angle - self.player.angle).abs().cos();

                let _ho = ((height as f32 / rd).floor() - 1.0) as usize;
                let mut ho = _ho;
                if ho > half_height {
                    // KOSTYL: Check for some float shenanigans.
                    ho = half_height;
                }

                let offset = half_height - ho;

                let ceil = offset;

                let floor = height - offset;

                let wall_sample = sample_wall(cx, cy, 1000.0);

                for i in ceil..floor {
                    let sx = (i-ceil + (_ho - ho)) as f32 / (2 * _ho) as f32 ;
                    *renderer.get_pixel_mut(i, px).unwrap() =
                        self.wall_shader
                            .sample(ray, sx, wall_sample);
                }


                for i in floor..height {
                    let ah = height - i;
                    *renderer.get_pixel_mut(i, px).unwrap() = self.floor_shader.sample(
                        (ah as f32) / half_height as f32,
                        1.0 - ((ah as f32) / half_height as f32),
                        px as f32 / width as f32,
                    );
                }

                for i in 0..ceil {
                    *renderer.get_pixel_mut(i, px).unwrap() = self.ceil_shader.sample(
                        i as f32 / half_height as f32,
                        i as f32 / half_height as f32,
                        px as f32 / width as f32,
                    );
                }
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
        let wall_texture = vec![
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
                    sprite: wall_texture,
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
