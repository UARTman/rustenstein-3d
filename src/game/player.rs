pub struct Player {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) angle: f32,
}

impl Player {
    fn new(x: f32, y: f32, angle: f32) -> Self {
        Self { x, y, angle }
    }

    pub fn go_forward(&mut self, d: f32) {
        self.x += d * -self.angle.sin();
        self.y += d * self.angle.cos();
    }

    pub fn go_right(&mut self, d: f32) {
        self.x += d * self.angle.cos();
        self.y += d * self.angle.sin();
    }

    pub fn rotate(&mut self, a: f32) {
        self.angle += a;
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            x: 10.0,
            y: 3.0,
            angle: 0.0,
        }
    }
}
