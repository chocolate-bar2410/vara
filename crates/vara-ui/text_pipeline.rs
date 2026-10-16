use std::ops::Mul;

use mlua::ExternalResult;

use crate::text_entry::TextEntry;

pub struct TextPipeline{
    font_system: glyphon::FontSystem,
    swash_cache: glyphon::SwashCache,
    text_atlas: glyphon::TextAtlas,
    viewport: glyphon::Viewport,
    text_renderer: glyphon::TextRenderer,
    text_buffers: Vec<glyphon::Buffer>,
    text_entries: Vec<TextEntry>,
    
    screen_width: f32,
    screen_height: f32,
}

impl TextPipeline{
    pub fn new(device: &wgpu::Device,queue: &wgpu::Queue,config: &wgpu::SurfaceConfiguration) -> Self{
        let font_system = glyphon::FontSystem::new();
        let swash_cache = glyphon::SwashCache::new();

        let cache = glyphon::Cache::new(&device);

        let mut text_atlas = glyphon::TextAtlas::new(&device, &queue, &cache, config.format);

        let viewport = glyphon::Viewport::new(&device, &cache);

        let text_renderer = glyphon::TextRenderer::new(&mut text_atlas, &device, wgpu::MultisampleState::default(), None);

        Self {
            font_system,
            swash_cache,
            text_atlas,
            viewport,
            text_renderer,
            text_buffers: Vec::new(),
            text_entries: Vec::new(),

            screen_width: 0.0,
            screen_height: 0.0,
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

    pub fn draw_text(&mut self,text_entry: TextEntry){
        if self.text_buffers.len() <= self.text_entries.len() {
            let text_buffer = glyphon::Buffer::new(
                &mut self.font_system, glyphon::Metrics { 
                    font_size: text_entry.text_size, 
                    line_height: text_entry.text_size * 1.25 
                }
            );
            self.text_buffers.push(text_buffer);
        }

        let text_buffer = &mut self.text_buffers[self.text_entries.len()];

        text_buffer.set_metrics(glyphon::Metrics { 
            font_size: text_entry.text_size, 
            line_height: text_entry.text_size * 1.25 
        });

        text_buffer.set_size(
            Some(self.screen_width * text_entry.size[0]), 
            Some(self.screen_height * text_entry.size[1])
        );
        text_buffer.set_text(&text_entry.text, &glyphon::Attrs::new(), glyphon::Shaping::Advanced,None);

        text_buffer.shape_until_scroll(&mut self.font_system, false);

        self.text_entries.push(text_entry);
    }

    pub fn resize(&mut self,width: f32,height: f32){
        self.screen_width = width;
        self.screen_height = height;
    }

    pub fn prepare(&mut self,device: &wgpu::Device,queue: &wgpu::Queue) -> mlua::Result<()> {
        
        let text_areas = self.text_buffers
        .iter()
        .zip(self.text_entries.iter())
        .map(|(buffer,entry)| {
            
            let x = (entry.position[0] + 1.0) * 0.5 * self.screen_width;
            let y = (1.0 - entry.position[1]) * 0.5 * self.screen_height;

            let width = entry.size[0] * self.screen_width;
            let height = entry.size[1] * self.screen_height;

            glyphon::TextArea{
                buffer,
                left: x,
                top: y ,
                scale: 1.0,
                bounds: glyphon::TextBounds {
                    left: x as i32,
                    top: y as i32,
                    right: (x + width)as i32,
                    bottom: (y + height) as i32,
                },
                default_color: glyphon::Color::rgba(
                    entry.colour[0].mul(255.0) as u8,
                    entry.colour[1].mul(255.0) as u8,
                    entry.colour[2].mul(255.0) as u8,
                    entry.colour[3].mul(255.0) as u8,
                ),
                custom_glyphs: &[],
            }
        });
        
        self.text_renderer.prepare(
            device, 
            queue, 
            &mut self.font_system, 
            &mut self.text_atlas, 
            &self.viewport, 
            text_areas,
            &mut self.swash_cache
        ).into_lua_err()
    }

    pub fn render(&mut self,render_pass: &mut wgpu::RenderPass)-> mlua::Result<()> {
        self.text_renderer.render(&self.text_atlas, &self.viewport, render_pass).into_lua_err()?;
        self.text_entries.clear();

        Ok(())
    }
}