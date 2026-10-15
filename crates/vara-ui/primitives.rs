
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex{
    pub position: [f32;2],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x2,
            }],
        }
    }
}

pub const QUAD_VERTICES : &[Vertex] = &[
    Vertex{position: [-1.0, 1.0]},
    Vertex{position: [-1.0,-1.0]},
    Vertex{position: [ 1.0,-1.0]},
    Vertex{position: [ 1.0, 1.0]},
];

pub const QUAD_INDICES : &[u16] = &[
    0, 1, 2,
    0, 2, 3
];

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Rect{
    pub position: [f32;2],
    pub size: [f32;2],
    pub colour: [f32;4],
}

impl Rect {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        const ATTRIBUITES: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![
            1 => Float32x2,
            2 => Float32x2,
            3 => Float32x4
        ];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Rect>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &ATTRIBUITES,
        }
    }
}