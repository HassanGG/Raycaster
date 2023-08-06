use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

use crate::graphics::{self, Graphics, Line, Rect};

const MOVE_AMOUNT: f32 = 0.01;
const ROTATE_AMOUNT: f32 = 10.0;
const MAP_SIZE: usize = 10;
const GAME_WIDTH: usize = 2;
const WALL_COLOR: [f32; 3] = [255.0, 255.0, 255.0];

pub struct Game {
    pub graphics: Graphics,
    player: Player,
    map: [[u8; MAP_SIZE]; MAP_SIZE],
}

#[derive(Debug)]
struct Player {
    pub pos: [f32; 2],
    color: [f32; 3],
    width: f32,
    rotation: f32,
}

impl Player {
    pub fn move_forward(&mut self) {
        let rad = self.rotation.to_radians();
        let hypot = MOVE_AMOUNT;

        self.pos[0] -= hypot * rad.sin();
        self.pos[1] += hypot * rad.cos();
    }

    pub fn move_backward(&mut self) {
        let rad = self.rotation.to_radians();
        let hypot = MOVE_AMOUNT;

        self.pos[0] += hypot * rad.sin();
        self.pos[1] -= hypot * rad.cos();
    }
}

impl Game {
    pub fn new(graphics: Graphics) -> Self {
        let player = Player {
            pos: [0.0, 0.0],
            width: 0.1,
            color: [29.0, 209.0, 113.0],
            rotation: 0.0,
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
        let wall_width = GAME_WIDTH as f32 / MAP_SIZE as f32;
        let n = self.map.len();
        for i in (0..n) {
            for j in (0..n) {
                if self.map[i][j] == 1 {
                    let origin = [
                        (i as f32 * wall_width) + (wall_width / 2.0) - 1.0,
                        (j as f32 * wall_width) + (wall_width / 2.0) - 1.0,
                    ];
                    let width = wall_width - 0.01;
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
                    self.player.move_forward();
                    true
                }
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Down),
                    ..
                } => {
                    self.player.move_backward();
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
            self.player.color,
            self.player.rotation,
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

