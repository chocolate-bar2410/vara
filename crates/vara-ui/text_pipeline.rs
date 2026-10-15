use mlua::ExternalResult;

pub struct TextPipeline{
    font_system: glyphon::FontSystem,
    swash_cache: glyphon::SwashCache,
    text_atlas: glyphon::TextAtlas,
    viewport: glyphon::Viewport,
    text_renderer: glyphon::TextRenderer,
    text_buffer: glyphon::Buffer
}

impl TextPipeline{
    pub fn new(device: &wgpu::Device,queue: &wgpu::Queue,config: &wgpu::SurfaceConfiguration) -> Self{
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

        Self {
            font_system,
            swash_cache,
            text_atlas,
            viewport,
            text_renderer,
            text_buffer,
        }
    }

    pub fn update(&mut self,queue: &wgpu::Queue,config: &wgpu::SurfaceConfiguration){
        self.viewport.update(
            queue,glyphon::Resolution{
                width: config.width,
                height: config.height
            }
        );
    }

    pub fn prepare(&mut self,device: &wgpu::Device,queue: &wgpu::Queue,config: &wgpu::SurfaceConfiguration) -> mlua::Result<()> {
        self.text_renderer.prepare(
            device, 
            queue, 
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
                    right: config.width as i32,
                    bottom: config.height as i32,
                },
                default_color: glyphon::Color::rgb(255,255,255),
                custom_glyphs: &[],
            }],
            &mut self.swash_cache
        ).into_lua_err()
    }

    pub fn render(&self,render_pass: &mut wgpu::RenderPass)-> mlua::Result<()> {
        self.text_renderer.render(&self.text_atlas, &self.viewport, render_pass).into_lua_err()
    }
}