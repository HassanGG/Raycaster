use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

use crate::{
    graphics::{self, Direction, Graphics, Line, Rect},
    player::{Player, LINE_LENGTH},
};

pub const MOVE_AMOUNT: f32 = 0.01;
pub const ROTATE_AMOUNT: f32 = 10.0;
pub const MAP_SIZE: usize = 10;
pub const GAME_WIDTH: usize = 2;
pub const WALL_COLOR: [f32; 3] = [255.0, 255.0, 255.0];
pub const WALL_WIDTH: f32 = GAME_WIDTH as f32 / MAP_SIZE as f32;
pub const PLAYER_COLOR: [f32; 3] = [0.0, 255.0, 0.0];
pub type GameMap = [[u8; MAP_SIZE]; MAP_SIZE];

pub struct Game {
    pub graphics: Graphics,
    player: Player,
    map: GameMap,
}

impl Game {
    pub fn new(graphics: Graphics) -> Self {
        let pos = [0.0, 0.0];
        let width = 0.1;
        let view = Direction {
            origin: pos,
            rotation: 0.0,
            length: LINE_LENGTH,
        };
        let player = Player {
            pos,
            width,
            rotation: 0.0,
            view,
        };

        let map = [
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 1, 1, 0, 0, 0, 0, 0, 1],
            [1, 0, 1, 1, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];

        Self {
            graphics,
            player,
            map,
        }
    }

    fn draw_map(&mut self) {
        let n = self.map.len();
        for i in (0..n) {
            for j in (0..n) {
                if self.map[i][j] == 1 {
                    let origin = [
                        (i as f32 * WALL_WIDTH) + (WALL_WIDTH / 2.0) - 1.0,
                        (j as f32 * WALL_WIDTH) + (WALL_WIDTH / 2.0) - 1.0,
                    ];
                    let width = WALL_WIDTH - 0.01;
                    let color = WALL_COLOR;
                    let rotation = 0.0;

                    self.graphics.push_square(origin, width, color, rotation)
                }
            }
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput { input, .. } => match input {
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Up),
                    ..
                } => {
                    self.player.move_forward(self.map);
                    true
                }
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Down),
                    ..
                } => {
                    self.player.move_backward(self.map);
                    true
                }

                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Right),
                    ..
                } => {
                    self.player.rotation -= ROTATE_AMOUNT;
                    true
                }
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Left),
                    ..
                } => {
                    self.player.rotation += ROTATE_AMOUNT;
                    true
                }

                _ => false,
            },
            _ => false,
        }
    }
    fn push_player(&mut self) {
        self.graphics.push_square(
            self.player.pos,
            self.player.width,
            PLAYER_COLOR,
            self.player.rotation,
        );

        self.graphics.push_direction(
            self.player.pos,
            LINE_LENGTH,
            self.player.rotation,
            PLAYER_COLOR,
        );
    }

    pub fn update(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.draw_map();
        self.push_player();
        let err = self.graphics.draw();
        self.graphics.clear();
        err
    }
}

