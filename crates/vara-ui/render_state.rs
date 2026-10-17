use std::sync::Arc;

use mlua::{ExternalError, ExternalResult};
use winit::{dpi::LogicalSize, event_loop::ActiveEventLoop, keyboard::KeyCode, window::Window};

use crate::{primitives::Rect, rect_pipeline::RectPipeline, text_entry::TextEntry, text_pipeline::TextPipeline};


pub struct RenderState{
    pub window: Arc<Window>,

    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,

    rect_pipeline: RectPipeline,
    text_pipeline: TextPipeline
}

impl RenderState {
    pub async fn new(window: Arc<Window>) -> mlua::Result<Self>{
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor{
            backends: wgpu::Backends::PRIMARY,
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });

        let surface = instance.create_surface(window.clone()).into_lua_err()?;

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptionsBase { 
            power_preference: wgpu::PowerPreference::default(), 
            force_fallback_adapter: false, 
            compatible_surface: Some(&surface), 
            apply_limit_buckets: true, 
        }).await.into_lua_err()?;

        let (device,queue) = adapter.request_device(&wgpu::wgt::DeviceDescriptor { 
            label: None, 
            required_features: wgpu::Features::empty(), 
            required_limits: wgpu::Limits::defaults(), 
            experimental_features: wgpu::ExperimentalFeatures::disabled(), memory_hints: Default::default(), trace: wgpu::Trace::Off 
        }).await.into_lua_err()?;

        let surface_capabilities = surface.get_capabilities(&adapter);

        let surface_format =  surface_capabilities.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_capabilities.formats[0]);

        let config = wgpu::SurfaceConfiguration{
             usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            color_space: wgpu::SurfaceColorSpace::Auto,
        };

        let rect_pipeline = RectPipeline::new(&device, &config);

        let mut text_pipeline = TextPipeline::new(&device, &queue, &config);
        text_pipeline.resize(config.width as f32, config.height as f32);

        let is_surface_configured = config.width > 0 && config.height > 0;

        if is_surface_configured {
            surface.configure(&device, &config);
        }

        Ok(Self { 
            window, 
            surface, 
            device, 
            queue, 
            config, 
            is_surface_configured: is_surface_configured,
            rect_pipeline,
            text_pipeline
        })
    }

    pub fn resize(&mut self,width: u32,height: u32){
        if height == 0 || width == 0 {return}

        self.config.width = width;
        self.config.height = height;

        self.surface.configure(&self.device, &self.config);
        self.is_surface_configured = true;

        self.text_pipeline.resize(width as f32, height as f32);
    }

    pub fn handle_key(&self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        match (code, is_pressed) {
            (KeyCode::Escape, true) => event_loop.exit(),
            _ => {}
        }
    }

    pub fn render(&mut self) -> mlua::Result<()> {
        if !self.is_surface_configured {
            return Ok(());
        }

        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => {
                surface_texture
            }
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => {
                return Ok(());
            }

            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                return Err("Lost device".into_lua_err());
            }
        };

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Encoder")
        });

        self.text_pipeline.update(&self.queue, &self.config);
        self.text_pipeline.prepare(&self.device, &self.queue)?;

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor{
                label: Some("Render Pass"),

                color_attachments: &[Some(wgpu::RenderPassColorAttachment{
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color{
                            r: 0.01,
                            g: 0.01,
                            b: 0.01,
                            a: 1.0,
                        }),

                        store: wgpu::StoreOp::Store
                    }
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });

            self.rect_pipeline.register(&self.queue);
            self.rect_pipeline.render(&mut render_pass);

            self.text_pipeline.render(&mut render_pass)?;
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        self.queue.present(output);

        Ok(())
    }

    pub fn draw_rect(&mut self,rect: Rect){
        self.rect_pipeline.draw_rect(rect);
    }

    pub fn draw_text(&mut self,text: TextEntry){
        self.text_pipeline.draw_text(text);
    }

}