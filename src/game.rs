use crate::graphics::{self, Graphics, Rect, Line};

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
            .push_square(self.player.pos, self.player.width, self.player.color);
    }

    pub fn update(&mut self) -> Result<(), wgpu::SurfaceError> {
        // self.push_player();
        let line = Line {
            start: [0.0, 0.0],
            end: [0.5, 0.8],
            width:0.05,
        };
        self.graphics.push_line(line);
        let err = self.graphics.draw();
        self.graphics.clear();
        err
    }
}
