use std::vec;

use crate::{gpu::WGPUState, vertex::Vertex};

pub struct Graphics {
    pub gpu_state: WGPUState,
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
}

#[derive(Debug, Clone, Copy)]
pub struct Square {
    pub tl: [f32; 2],
    pub bl: [f32; 2],
    pub br: [f32; 2],
    pub tr: [f32; 2],
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

    pub fn push_square(&mut self, square: Square, color: [f32; 3]) {
        let length = self.vertices.len() as u16;
        let offset = if length > 0 { length - 1 } else { 0 };

        self.vertices.push(Vertex {
            position: square.tl,
            color,
        });
        self.vertices.push(Vertex {
            position: square.bl,
            color,
        });
        self.vertices.push(Vertex {
            position: square.br,
            color,
        });
        self.vertices.push(Vertex {
            position: square.tr,
            color,
        });

        self.indices.extend_from_slice(&[
            0 + offset, 1 + offset, 2 + offset,
            3 + offset, 0 + offset, 2 + offset,
        ]);

    }

    pub fn draw(&mut self) {
        self.gpu_state
            .update(self.vertices.as_slice(), self.indices.as_slice());
    }
}
