use crate::graphics::{self, Graphics, Line, Rect};

pub struct Game {
    pub graphics: Graphics,
    player: Player,
}

struct Player {
    pub pos: [f32; 2],
    color: [f32; 3],
    width: f32,
}

impl Game {
    pub fn new(graphics: Graphics) -> Self {
        let player = Player {
            pos: [0.0, 0.0],
            width: 0.1,
            color: [29.0, 209.0, 113.0],
        };

        Self { graphics, player }
    }

    fn push_player(&mut self) {
        self.graphics
            .push_square(self.player.pos, self.player.width, self.player.color, 0.0);
    }

    pub fn update(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.graphics
            .push_square([0.0, 0.0], 0.1, [0.0, 0.0, 255.0], 365.0);

        let err = self.graphics.draw();
        self.graphics.clear();
        err
    }
}
