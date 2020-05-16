pub struct Player {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) angle: f32,
}

impl Player {
    fn new(x: f32, y: f32, angle: f32) -> Self {
        Self {
            x,
            y,
            angle,
        }
    }

    pub fn go(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }

    pub fn rotate(&mut self, a: f32) {
        self.angle += a;
    }
}

impl Default for Player{
    fn default() -> Self {
        Self {
            x: 10.0,
            y: 3.0,
            angle: 0.0
        }
    }
}