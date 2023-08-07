use std::f32::consts::PI;

use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

use crate::{
    graphics::{self, Graphics, Line, Ray, Rect},
    player::{Player, LINE_LENGTH},
    util::convert_range,
};

pub const MOVE_AMOUNT: f32 = 0.01;
pub const ROTATE_AMOUNT: f32 = 10.0;
pub const MAP_SIZE: usize = 10;
pub const GAME_WIDTH: usize = 2;
pub const WALL_COLOR: [f32; 3] = [255.0, 255.0, 255.0];
pub const WALL_WIDTH: f32 = GAME_WIDTH as f32 / MAP_SIZE as f32;
pub const PLAYER_COLOR: [f32; 3] = [0.0, 255.0, 0.0];
pub const PLAYER_WIDTH: f32 = 0.03;
pub type GameMap = [[u8; MAP_SIZE]; MAP_SIZE];

pub struct Game {
    pub graphics: Graphics,
    player: Player,
    map: GameMap,
    ray_lengths: Vec<f32>,
}

impl Game {
    pub fn new(graphics: Graphics) -> Self {
        let mut pos = [0.0, 0.0];
        pos[0] = convert_range(pos[0], [-1.0, 1.0], [-1.0, 0.0]);
        let width = PLAYER_WIDTH;
        let ray_lengths: Vec<f32> = vec![];
        let view = Ray {
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
            ray_lengths,
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
                    if self.player.rotation < 0.0 {
                        self.player.rotation = 360.0 - ROTATE_AMOUNT;
                    }
                    self.player.rotation %= 360.0;
                    true
                }
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Left),
                    ..
                } => {
                    self.player.rotation += ROTATE_AMOUNT;
                    self.player.rotation %= 360.0;
                    true
                }

                _ => false,
            },
            _ => false,
        }
    }

    fn draw_rays(&mut self) {
        for mut deg in (-45..=45).step_by(1) {
            let angle = deg as f32 / 2.0;
            let mut ray = Ray {
                rotation: self.player.rotation + angle as f32,
                origin: self.player.pos,
                length: 0.0,
            };

            let end = ray.get_collision(self.map, 0.05);
            let start = self.player.pos;

            let mut ray_length =
                f32::sqrt((f32::powi(end[0] - start[0], 2)) + f32::powi((end[1] - start[1]), 2));

            let angle = self.player.rotation.to_radians() - ray.rotation.to_radians();
            ray_length = ray_length * (angle as f32).cos();
            self.ray_lengths.push(ray_length);

            let color = [255.0, 0.0, 255.0];
            self.graphics.push_line(
                Line {
                    start: self.player.pos,
                    end,
                },
                color,
            );
        }
        println!("{:#?}", self.ray_lengths);
    }

    fn draw_walls(&mut self) {
        let n = self.ray_lengths.len();
        let column_width = GAME_WIDTH as f32 / n as f32;
        let color = [255.0, 0.0, 0.0];

        for i in 0..n {
            let height = 2.0 - *self.ray_lengths.get(i).unwrap_or(&0.0);
            let rect = Rect {
                origin: [-1.0 + (column_width * i as f32), 0.0],
                rotation: 0.0,
                height: 2.0 - *self.ray_lengths.get(i).unwrap(),
                width: column_width,
            };
            self.graphics.push_rect_right(rect, color)
        }
    }

    fn push_player(&mut self) {
        self.graphics.push_square(
            self.player.pos,
            self.player.width,
            PLAYER_COLOR,
            self.player.rotation,
        );

        self.graphics.push_ray(
            Ray {
                origin: self.player.pos,
                length: LINE_LENGTH,
                rotation: self.player.rotation,
            },
            PLAYER_COLOR,
        );
    }

    pub fn update(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.draw_map();
        self.draw_rays();
        self.push_player();
        self.draw_walls();
        let err = self.graphics.draw();
        self.graphics.clear();
        self.ray_lengths.clear();
        err
    }
}
