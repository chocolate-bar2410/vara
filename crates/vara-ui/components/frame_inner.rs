use crate::{components::Renderable, primitives::Rect, render_state::RenderState};

pub struct UiFrameInner{
    pub visible: bool,
    pub position: [f32;2],
    pub size: [f32;2],
    pub colour: [f32;4],

    pub corner_radius: f32,
    pub border_width: f32,
    pub border_colour: [f32;4],
}

impl UiFrameInner {

    pub fn new() -> Self{
        Self {
            visible: true,
            position: [0.0,0.0],
            size: [0.1,0.1],
            colour: [0.5,0.5,0.5,1.0],
            corner_radius: 0.0, 
            border_width: 0.0, 
            border_colour: [0.0,0.0,0.0,1.0] 
        }
    }
    
}

impl Renderable for UiFrameInner{
    fn render(&self,render_state: &mut RenderState) -> mlua::Result<()>  {
        if !self.visible {return Ok(())}

        let x = self.position[0] * 2.0 - 1.0;
        let y = 1.0 - self.position[1] * 2.0;

        let width = self.size[0] * 2.0;
        let height = self.size[1] * 2.0;

        render_state.draw_rect(Rect { 
            position: [x,y], 
            size: [width,height], 
            colour: self.colour, 
            corner_radius: self.corner_radius, 
            border_width: self.border_width,
            border_colour: self.border_colour,
        });
     
        Ok(())
    }
}