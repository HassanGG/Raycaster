use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

use crate::graphics::{self, Graphics, Line, Rect};

const MOVE_AMOUNT: f32 = 0.01;
const ROTATE_AMOUNT: f32 = 10.0;
const MAP_SIZE: usize = 10;
const GAME_WIDTH: usize = 2;
const WALL_COLOR: [f32; 3] = [255.0, 255.0, 255.0];
const WALL_WIDTH: f32 = GAME_WIDTH as f32 / MAP_SIZE as f32;
type Map = [[u8; MAP_SIZE]; MAP_SIZE];

pub struct Game {
    pub graphics: Graphics,
    player: Player,
    map: Map,
}

#[derive(Debug)]
struct Player {
    pub pos: [f32; 2],
    color: [f32; 3],
    width: f32,
    rotation: f32,
}

impl Player {
    fn validate_move(&mut self, x: f32, y: f32, map: Map) -> bool {
        let n = map.len();
        for i in (0..n) {
            for j in (0..n) {
                if map[i][j] == 1 {
                    let (beg_x, end_x) = (
                        i as f32 * WALL_WIDTH - 1.0,
                        (i as f32 + 1.0) * WALL_WIDTH - 1.0,
                    );
                    let (beg_y, end_y) = (
                        j as f32 * WALL_WIDTH - 1.0,
                        (j as f32 + 1.0) * WALL_WIDTH - 1.0,
                    );
                    let h_p_width = (self.width - 0.05) / 2.0;
                    if x + h_p_width < end_x
                        && x + h_p_width > beg_x
                        && y + h_p_width > beg_y
                        && y + h_p_width < end_y
                    {
                        return false;
                    }
                    if x - h_p_width < end_x
                        && x - h_p_width > beg_x
                        && y - h_p_width > beg_y
                        && y - h_p_width < end_y
                    {
                        return false;
                    }
                    if x + h_p_width < end_x
                        && x + h_p_width > beg_x
                        && y - h_p_width > beg_y
                        && y - h_p_width < end_y
                    {
                        return false;
                    }
                    if x - h_p_width < end_x
                        && x - h_p_width > beg_x
                        && y + h_p_width > beg_y
                        && y + h_p_width < end_y
                    {
                        return false;
                    }
                }
            }
        }

        true
    }
    pub fn move_forward(&mut self, map: Map) {
        let rad = self.rotation.to_radians();
        let hypot = MOVE_AMOUNT;

        let x = self.pos[0] - hypot * rad.sin();
        let y = self.pos[1] + hypot * rad.cos();
        if self.validate_move(x, y, map) {
            self.pos[0] = x;
            self.pos[1] = y;
        }
    }

    pub fn move_backward(&mut self, map: Map) {
        let rad = self.rotation.to_radians();
        let hypot = MOVE_AMOUNT;

        let x = self.pos[0] + hypot * rad.sin();
        let y = self.pos[1] - hypot * rad.cos();
        if self.validate_move(x, y, map) {
            self.pos[0] = x;
            self.pos[1] = y;
        }
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

