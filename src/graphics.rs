use crate::{
    game::{GameMap, MAP_SIZE, WALL_WIDTH},
    gpu::{WGPUState, MAX_INDICES, MAX_VERTICES},
    util::convert_range,
    vertex::Vertex,
};

pub struct Graphics {
    pub gpu_state: WGPUState,
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    lines: Vec<Vertex>,
}

#[derive(Debug, Clone, Copy)]
pub struct Quad {
    pub tl: [f32; 2],
    pub bl: [f32; 2],
    pub br: [f32; 2],
    pub tr: [f32; 2],
}

pub struct Rect {
    pub origin: [f32; 2],
    pub rotation: f32,
    pub height: f32,
    pub width: f32,
}

#[derive(Debug)]
pub struct Ray {
    pub origin: [f32; 2],
    pub length: f32,
    pub rotation: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub start: [f32; 2],
    pub end: [f32; 2],
}

impl Ray {
    pub fn get_collision(&mut self, map: GameMap, march: f32) -> [f32; 2] {
        let mut collided = self.get_collided(map);

        let max_iter = 30;
        let mut i = 0;
        while collided.is_none() && i < max_iter {
            let rad = self.rotation.to_radians();
            let hypot = march;

            let x = self.origin[0] - hypot * rad.sin();
            let y = self.origin[1] + hypot * rad.cos();
            self.origin = [x, y];

            collided = self.get_collided(map);
            i += 1
        }

        println!("{:#?}", self.origin);
        self.origin
    }

    fn is_collided(&self, i: usize, j: usize) -> bool {
        let mut x_min = WALL_WIDTH * j as f32;
        let mut x_max = (WALL_WIDTH * j as f32) + WALL_WIDTH;
        let mut y_min = WALL_WIDTH * i as f32;
        let mut y_max = (WALL_WIDTH * i as f32) + WALL_WIDTH;

        x_min = convert_range(x_min, [0.0, 2.0], [-1.0, 1.0]);
        y_min = convert_range(y_min, [0.0, 2.0], [-1.0, 1.0]);
        y_max = convert_range(y_max, [0.0, 2.0], [-1.0, 1.0]);
        x_max = convert_range(x_max, [0.0, 2.0], [-1.0, 1.0]);

        self.origin[0] <= x_max
            && self.origin[0] >= x_min
            && self.origin[1] <= y_max
            && self.origin[1] >= y_min
    }

    fn get_collided(&self, map: GameMap) -> Option<[f32; 2]> {
        for i in 0..map.len() {
            for j in 0..map[0].len() {
                if map[i][j] == 1 {
                    if self.is_collided(i, j) {
                        return Some([i as f32, j as f32]);
                    }
                }
            }
        }

        None
    }
}

impl Line {
    pub fn new(start: [f32; 2], end: [f32; 2]) -> Self {
        Self { start, end }
    }
    pub fn rotate(&mut self, rotation: f32) {
        let rad = rotation.to_radians();
        let rotate_point = |x: f32, y: f32| -> [f32; 2] {
            [
                x * f32::cos(rad) - y * f32::sin(rad),
                x * f32::sin(rad) + y * f32::cos(rad),
            ]
        };

        self.start = rotate_point(self.start[0], self.start[1]);
        self.end = rotate_point(self.end[0], self.end[1]);
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        let t_point = |p: [f32; 2]| -> [f32; 2] { [p[0] + x, p[1] + y] };

        self.start = t_point(self.start);
        self.end = t_point(self.end);
    }
}

impl Quad {
    pub fn translate(&mut self, x: f32, y: f32) {
        let t_point = |p: [f32; 2]| -> [f32; 2] { [p[0] + x, p[1] + y] };

        self.tr = t_point(self.tr);
        self.bl = t_point(self.bl);
        self.br = t_point(self.br);
        self.tl = t_point(self.tl);
    }

    pub fn rotate(&mut self, rotation: f32) {
        let rad = rotation.to_radians();
        let rotate_point = |x: f32, y: f32| -> [f32; 2] {
            [
                x * f32::cos(rad) - y * f32::sin(rad),
                x * f32::sin(rad) + y * f32::cos(rad),
            ]
        };
        self.tl = rotate_point(self.tl[0], self.tl[1]);
        self.tr = rotate_point(self.tr[0], self.tr[1]);
        self.bl = rotate_point(self.bl[0], self.bl[1]);
        self.br = rotate_point(self.br[0], self.br[1]);
    }
}

impl Graphics {
    pub fn new(gpu_state: WGPUState) -> Self {
        Self {
            gpu_state,
            vertices: Vec::with_capacity(MAX_VERTICES as usize),
            lines: Vec::with_capacity(MAX_VERTICES as usize),
            indices: Vec::with_capacity(MAX_INDICES as usize),
        }
    }

    pub fn clear(&mut self) {
        self.indices.clear();
        self.lines.clear();
        self.vertices.clear();
    }

    fn offset(&self) -> u16 {
        let length = self.vertices.len() as u16;
        if length > 0 {
            length
        } else {
            0
        }
    }

    pub fn push_square(&mut self, origin: [f32; 2], width: f32, color: [f32; 3], rotation: f32) {
        let square = Rect {
            width,
            height: width,
            origin,
            rotation,
        };
        self.push_rect(square, color);
    }

    pub fn push_ray(&mut self, ray: Ray, color: [f32; 3]) {
        let mut line = Line {
            start: [0.0, 0.0],
            end: [0.0, 0.0 + ray.length],
        };

        line.rotate(ray.rotation);
        line.translate(ray.origin[0], ray.origin[1]);

        self.push_line(line, color);
    }

    pub fn push_line(&mut self, line: Line, color: [f32; 3]) {
        let ap = self.gpu_state.aspect_ratio;

        self.lines.extend_from_slice(&[
            Vertex {
                position: [line.start[0] * ap, line.start[1]],
                color,
            },
            Vertex {
                position: [line.end[0] * ap, line.end[1]],
                color,
            },
        ]);
    }

    fn push_quad(&mut self, quad: Quad, color: [f32; 3]) {
        let offset = self.offset();
        let ap = self.gpu_state.aspect_ratio;

        self.vertices.extend_from_slice(&[
            Vertex {
                position: [quad.tl[0] * ap, quad.tl[1]],
                color,
            },
            Vertex {
                position: [quad.bl[0] * ap, quad.bl[1]],
                color,
            },
            Vertex {
                position: [quad.br[0] * ap, quad.br[1]],
                color,
            },
            Vertex {
                position: [quad.tr[0] * ap, quad.tr[1]],
                color,
            },
        ]);

        self.indices.extend_from_slice(&[
            0 + offset,
            1 + offset,
            2 + offset,
            3 + offset,
            0 + offset,
            2 + offset,
        ]);
    }

    pub fn push_rect(&mut self, rect: Rect, color: [f32; 3]) {
        let hw = rect.width / 2.0;
        let hh = rect.height / 2.0;

        let mut quad = Quad {
            tl: [-hw, hh],
            bl: [-hw, -hh],
            br: [hw, -hh],
            tr: [hw, hh],
        };

        quad.rotate(rect.rotation);
        quad.translate(rect.origin[0], rect.origin[1]);
        self.push_quad(quad, color);
    }

    pub fn draw(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.gpu_state
            .update_tri(self.vertices.as_slice(), self.indices.as_slice());
        self.gpu_state.update_line(self.lines.as_slice());
        self.gpu_state.render()
    }
}
