use winit::event::{ElementState, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};

use crate::ray::Ray;
use crate::util::convert_range;
use crate::{
    graphics::{Graphics, Rect},
    player::{Player, LINE_LENGTH},
    ray::Lighting,
};

pub const MOVE_AMOUNT: f32 = 0.01;
pub const ROTATE_AMOUNT: f32 = 10.0;
pub const MAP_SIZE: usize = 10;
pub const COORD_SIZE: usize = 2;
pub const WALL_COLOR: [f32; 3] = [255.0, 255.0, 255.0];
pub const CELL_WIDTH: f32 = COORD_SIZE as f32 / MAP_SIZE as f32;
pub const PLAYER_COLOR: [f32; 3] = [0.0, 255.0, 0.0];
pub const GROUND_COLOR: [f32; 3] = [0.1, 0.30, 0.0];
pub const SKY_COLOR: [f32; 3] = [0.2, 0.4, 1.0];
pub const PLAYER_WIDTH: f32 = 0.03;
pub const SHADED_COLOR: [f32; 3] = [0.5, 0.0, 0.0];
pub const LIT_COLOR: [f32; 3] = [1.0, 0.0, 0.0];
pub type GameMap = [[u8; MAP_SIZE]; MAP_SIZE];

pub struct Game {
    pub graphics: Graphics,
    player: Player,
    map: GameMap,
    ray_data: Vec<(f32, Lighting)>,
    mouse_location: [f32; 2],
    mouse_left: bool,
    mouse_right: bool,
}

enum HandleWall {
    Destroy,
    Create,
}

impl Game {
    pub fn new(graphics: Graphics) -> Self {
        let pos = [0.0, 0.0];
        let width = PLAYER_WIDTH;
        let ray_lengths: Vec<(f32, Lighting)> = vec![];
        let mouse_location = [0.0, 0.0];
        let mouse_left = false;
        let mouse_right = false;
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

        let map: GameMap = [
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];

        assert!(map.len() == MAP_SIZE);
        Self {
            graphics,
            mouse_right,
            mouse_left,
            player,
            map,
            ray_data: ray_lengths,
            mouse_location,
        }
    }

    fn draw_map(&mut self) {
        let n = self.map.len();
        for i in 0..n {
            for j in 0..n {
                if self.map[i][j] == 1 {
                    let origin = [
                        (i as f32 * CELL_WIDTH) + (CELL_WIDTH / 2.0) - 1.0,
                        (j as f32 * CELL_WIDTH) + (CELL_WIDTH / 2.0) - 1.0,
                    ];
                    let width = CELL_WIDTH - 0.01;
                    let color = WALL_COLOR;
                    let rotation = 0.0;

                    self.graphics.push_square(origin, width, color, rotation)
                }
            }
        }
    }

    fn handle_wall(&mut self, handle: HandleWall) {
        let n = self.map.len() as f32;
        let width = self.graphics.gpu_state.size.width as f32;
        let height = self.graphics.gpu_state.size.height as f32;
        let x = convert_range(self.mouse_location[0], [0.0, width], [0.0, n * 2.0]) as usize;
        let y = n as usize
            - 1
            - convert_range(self.mouse_location[1], [0.0, height], [0.0, n]) as usize;

        let player_x = convert_range(self.player.pos[0], [-1.0, 1.0], [0.0, n]);
        let player_y = convert_range(self.player.pos[1], [-1.0, 1.0], [0.0, n]);

        if player_x <= ( x+1 ) as f32 && player_x >= x as f32 && player_y <= (y+1) as f32 && player_y >= y as f32 {
            return;

        }
        if x >= (n - 1.0) as usize || y >= (n - 1.0) as usize || x == 0 || y == 0 {
            return;
        }

        self.map[x][y] = match handle {
            HandleWall::Destroy => 0,
            HandleWall::Create => 1,
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_location = [position.x as f32, position.y as f32];
                if self.mouse_left {
                    self.handle_wall(HandleWall::Create);
                }
                if self.mouse_right {
                    self.handle_wall(HandleWall::Destroy);
                }
                true
            }

            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                self.handle_wall(HandleWall::Create);
                self.mouse_left = true;
                true
            }

            WindowEvent::MouseInput {
                state: ElementState::Released,
                button: MouseButton::Left,
                ..
            } => {
                self.mouse_left = false;
                true
            }

            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Right,
                ..
            } => {
                self.handle_wall(HandleWall::Destroy);
                self.mouse_right = true;
                true
            }

            WindowEvent::MouseInput {
                state: ElementState::Released,
                button: MouseButton::Right,
                ..
            } => {
                self.mouse_right = false;
                true
            }

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

    fn cast_rays(&mut self) {
        let view_angle = 60;
        for deg in (-view_angle..=view_angle).step_by(1) {
            let angle = (deg as f32 / 2.0) % 360.0;
            let ray = Ray {
                rotation: self.player.rotation + angle as f32,
                origin: self.player.pos,
                length: 0.0,
            };
            let (ray_length, lighting) = ray.collision(self.map);
            let color = match lighting {
                Lighting::Shaded => SHADED_COLOR,
                Lighting::Lit => LIT_COLOR,
            };
            let fix_fisheye = |length: f32| length * (angle as f32).to_radians().cos();
            self.ray_data.push((fix_fisheye(ray_length), lighting));
            self.graphics.push_ray(
                Ray {
                    length: ray_length,
                    origin: self.player.pos,
                    rotation: self.player.rotation + angle,
                },
                color,
            )
        }
    }

    fn draw_walls(&mut self) {
        let ground = Rect {
            rotation: 0.0,
            origin: [0.0, -0.5],
            height: 1.0,
            width: 2.0,
        };
        let sky = Rect {
            rotation: 0.0,
            origin: [0.0, 0.5],
            height: 1.0,
            width: 2.0,
        };
        self.graphics.push_rect_right(ground, GROUND_COLOR);
        self.graphics.push_rect_right(sky, SKY_COLOR);
        let n = self.ray_data.len();
        let column_width = COORD_SIZE as f32 / n as f32;
        for i in 0..n {
            let (ray_length, lighting) = self.ray_data.get(i).unwrap_or(&(0.0, Lighting::Lit));
            let color = match lighting {
                Lighting::Shaded => SHADED_COLOR,
                Lighting::Lit => LIT_COLOR,
            };
            let height = 0.5 / ray_length;
            let rect = Rect {
                origin: [
                    (-1.0 + (column_width / 2.0)) + (column_width * i as f32),
                    0.0,
                ],
                rotation: 0.0,
                height,
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
        let ray = Ray {
            origin: self.player.pos,
            length: LINE_LENGTH,
            rotation: self.player.rotation,
        };
        self.graphics.push_ray(ray, PLAYER_COLOR);
    }

    pub fn update(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.draw_map();
        self.push_player();
        self.cast_rays();
        self.draw_walls();
        let err = self.graphics.draw();
        self.graphics.clear();
        self.ray_data.clear();
        err
    }
}
