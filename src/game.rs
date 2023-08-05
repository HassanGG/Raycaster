use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

use crate::graphics::{self, Graphics, Line, Rect};

pub struct Game {
    pub graphics: Graphics,
    player: Player,
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

const MOVE_AMOUNT: f32 = 0.01;
const ROTATE_AMOUNT: f32 = 8.0;

impl Game {
    pub fn new(graphics: Graphics) -> Self {
        let player = Player {
            pos: [0.0, 0.0],
            width: 0.1,
            color: [29.0, 209.0, 113.0],
            rotation: 0.0,
        };

        Self { graphics, player }
    }

    fn draw_map(&mut self) {}

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput { input, .. } => match input {
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Up),
                    ..
                } => {
                    // self.player.pos[1] += MOVE_AMOUNT;
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
        self.push_player();
        let err = self.graphics.draw();
        self.graphics.clear();
        err
    }
}

