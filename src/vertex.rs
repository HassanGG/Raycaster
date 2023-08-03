#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5],
        color: [0.5, 0.0, 0.5],
    }, 
    Vertex {
        position: [0.0, -0.5],
        color: [0.5, 0.0, 0.5],
    }, 
    Vertex {
        position: [0.5, -0.5],
        color: [0.5, 0.0, 0.5],
    }, 
    Vertex {
        position: [0.5, 0.5],
        color: [0.5, 0.0, 0.5],
    }, 
];

pub const INDICES: &[u16] = &[
    0, 1, 2,
    3, 0, 2,
];

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}