use std::vec;

use crate::{gpu::WGPUState, vertex::Vertex};

pub struct Graphics {
    pub gpu_state: WGPUState,
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub tl: [f32; 2],
    pub bl: [f32; 2],
    pub br: [f32; 2],
    pub tr: [f32; 2],
}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub start: [f32; 2],
    pub end: [f32; 2],
    pub width: f32,
}

impl Graphics {
    pub fn new(gpu_state: WGPUState) -> Self {
        Self {
            gpu_state,
            vertices: Vec::with_capacity(4000),
            indices: Vec::with_capacity(6000),
        }
    }

    pub fn clear(&mut self) {
        self.indices.clear();
        self.vertices.clear();
    }

    fn offset(&self) -> u16 {
        let length = self.vertices.len() as u16;
        if length > 0 {
            length - 1
        } else {
            0
        }
    }

    pub fn push_square(&mut self, pos: [f32; 2], width: f32, color: [f32; 3], rotation: f32) {
        let h = width / 2.0;
        let w = h;
        let square = Rect {
            tl: [pos[0] - w, pos[1] + h],
            bl: [pos[0] - w, pos[1] - h],
            tr: [pos[0] + w, pos[1] + h],
            br: [pos[0] + w, pos[1] - h],
        };

        self.push_rect(square, color, rotation);
    }

    pub fn push_line(&mut self, line: Line) {
        let w = line.width / 2.0;

        let x1 = line.start[0];
        let x2 = line.end[0];
        let y1 = line.start[1];
        let y2 = line.end[1];

        let color: [f32; 3] = [1.0, 1.0, 1.0];

        let dx = x2 - x1;
        let dy = y2 - y1;
        let l = dx.hypot(dy);
        let u = dx * line.width * 0.5 / l;
        let v = dy * line.width * 0.5 / l;

        self.vertices.push(Vertex {
            position: [x1 + v, y1 - u],
            color,
        });
        self.vertices.push(Vertex {
            position: [x1 - v, y1 + u],
            color,
        });
        self.vertices.push(Vertex {
            position: [x2 - v, y2 + u],
            color,
        });
        self.vertices.push(Vertex {
            position: [x2 + v, y2 - u],
            color,
        });

        self.indices.push(1);
        self.indices.push(2);
        self.indices.push(0);
        self.indices.push(2);
        self.indices.push(0);
        self.indices.push(3);
    }

    fn rotate_square(&self, rect: &mut Rect, rotation: f32) {
        let rad = rotation.to_radians();
        let rotate_point = |x: f32, y: f32| -> [f32; 2] {
            [
                x * f32::cos(rad) - y * f32::sin(rad),
                x * f32::sin(rad) + y * f32::cos(rad),
            ]
        };
        rect.tl = rotate_point(rect.tl[0], rect.tl[1]);
        rect.tr = rotate_point(rect.tr[0], rect.tr[1]);
        rect.bl = rotate_point(rect.bl[0], rect.bl[1]);
        rect.br = rotate_point(rect.br[0], rect.br[1]);
    }

    pub fn push_rect(&mut self, rect: Rect, color: [f32; 3], rotation: f32) {
        let offset = self.offset();
        let ap = self.gpu_state.aspect_ratio;
        let mut rect = rect;

        self.rotate_square(&mut rect, rotation);

        self.vertices.extend_from_slice(&[
            Vertex {
                position: [rect.tl[0] * ap, rect.tl[1]],
                color,
            },
            Vertex {
                position: [rect.bl[0] * ap, rect.bl[1]],
                color,
            },
            Vertex {
                position: [rect.br[0] * ap, rect.br[1]],
                color,
            },
            Vertex {
                position: [rect.tr[0] * ap, rect.tr[1]],
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

    pub fn draw(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.gpu_state
            .update(self.vertices.as_slice(), self.indices.as_slice());
        return self.gpu_state.render();
    }
}
