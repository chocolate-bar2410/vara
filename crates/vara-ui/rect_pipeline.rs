use crate::primitives::Rect;

use crate::primitives::QUAD_VERTICES;
use crate::primitives::QUAD_INDICES;
use crate::primitives::Vertex;

use wgpu::util::DeviceExt;

const INITIAL_BUFFER_SIZE: u64 = 1024 * 1024;

pub struct RectPipeline{
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,

    pipeline: wgpu::RenderPipeline,

    rects: Vec<Rect>
}

impl RectPipeline {
    pub fn new(device: &wgpu::Device,config: &wgpu::SurfaceConfiguration) -> Self{
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor{
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/rect.wgsl").into()),
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { 
            label: Some("Render Pipeline Layout"), 
            bind_group_layouts: &[], 
            immediate_size: 0 
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(Vertex::desc()), Some(Rect::desc())],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },

            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),

            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),

                polygon_mode: wgpu::PolygonMode::Fill,

                unclipped_depth: false,

                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache: None,
        });
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex buffer"),
            usage: wgpu::BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(QUAD_VERTICES),
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("index buffer"),
            usage: wgpu::BufferUsages::INDEX,
            contents: bytemuck::cast_slice(QUAD_INDICES),
        });

        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("instance buffer"),
            size: INITIAL_BUFFER_SIZE,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline: render_pipeline,
            vertex_buffer,
            index_buffer,
            instance_buffer,
            rects: Vec::new(),
        }
    }

    pub fn add_rect(&mut self,rect: Rect){
        self.rects.push(rect);
    }

    pub fn register(&self,queue: &wgpu::Queue){
        if self.rects.is_empty() {
            return;
        }

        queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&self.rects));
    }

    pub fn render(&self,render_pass: &mut wgpu::RenderPass){
        render_pass.set_pipeline(&self.pipeline);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        render_pass.draw_indexed(0..6, 0, 0..self.rects.len() as u32);
    }
}