use std::sync::Arc;

use mlua::{ExternalError, ExternalResult};
use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode, window::Window};

use crate::{primitives::Rect, rect_pipeline::RectPipeline};


pub struct RenderState{
    window: Arc<Window>,

    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,

    rect_pipeline: RectPipeline,

    // text rendering - will probably move this to its own object later
    font_system: glyphon::FontSystem,
    swash_cache: glyphon::SwashCache,
    text_atlas: glyphon::TextAtlas,
    viewport: glyphon::Viewport,
    text_renderer: glyphon::TextRenderer,
    text_buffer: glyphon::Buffer
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

        let mut rect_pipeline = RectPipeline::new(&device, &config);

        rect_pipeline.add_rect(Rect{
            position: [-0.98,0.98],
            size: [0.5,0.5],
            colour: [0.02,0.02,0.02,1.0],
            corner_radius: 0.01,
            border_width: 0.002,
            border_colour:  [0.2,0.2,0.2,1.0],
        });


        // text rendering

        let mut font_system = glyphon::FontSystem::new();
        let swash_cache = glyphon::SwashCache::new();

        let cache = glyphon::Cache::new(&device);

        let mut text_atlas = glyphon::TextAtlas::new(&device, &queue, &cache, config.format);

        let viewport = glyphon::Viewport::new(&device, &cache);

        let text_renderer = glyphon::TextRenderer::new(&mut text_atlas, &device, wgpu::MultisampleState::default(), None);
        let mut text_buffer = glyphon::Buffer::new(&mut font_system, glyphon::Metrics::new(24.0,30.0));

        text_buffer.set_size(Some(400.0), Some(500.0));
        text_buffer.set_text("testing 123", &glyphon::Attrs::new(), glyphon::Shaping::Advanced,None);
        text_buffer.shape_until_scroll(&mut font_system, false);

        Ok(Self { 
            window, 
            surface, 
            device, 
            queue, 
            config, 
            is_surface_configured: false,
            rect_pipeline,
            
            font_system,
            swash_cache,
            text_atlas,
            viewport,
            text_renderer,
            text_buffer,
        })
    }

    pub fn resize(&mut self,width: u32,height: u32){
        if height == 0 || width == 0 {return;}

        self.config.width = width;
        self.config.height = height;

        self.surface.configure(&self.device, &self.config);
        self.is_surface_configured = true;
    }

    pub fn handle_key(&self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        match (code, is_pressed) {
            (KeyCode::Escape, true) => event_loop.exit(),
            _ => {}
        }
    }

    pub fn render(&mut self) -> mlua::Result<()> {
        self.window.request_redraw();

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

        self.viewport.update(&self.queue,glyphon::Resolution{width: self.config.width,height: self.config.height});
        
        self.text_renderer.prepare(
            &self.device, 
            &self.queue, 
            &mut self.font_system, 
            &mut self.text_atlas, 
            &self.viewport, 
            [glyphon::TextArea {
                left: 20.0,
                top: 20.0,
                buffer: &self.text_buffer,
                scale: 1.0,
                bounds: glyphon::TextBounds{
                    left: 0,
                    top: 0,
                    right: self.config.width as i32,
                    bottom: self.config.height as i32,
                },
                default_color: glyphon::Color::rgb(255,255,255),
                custom_glyphs: &[],
            }],
            &mut self.swash_cache
        ).into_lua_err()?;

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

            self.text_renderer.render(&self.text_atlas, &self.viewport, &mut render_pass).into_lua_err()?;
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        self.queue.present(output);

        Ok(())
    }
}