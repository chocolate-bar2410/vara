use crate::components::Renderable;
use crate::primitives::Rect;
use crate::render_state::RenderState;
use crate::text_entry::TextEntry;
pub struct UiTextLabelInner{
    pub text: String,
    pub text_size: f32,

    pub visible: bool,
    pub position: [f32;2],
    pub size: [f32;2],
    pub text_colour: [f32;4],
    pub background_colour: [f32;4],

    pub corner_radius: f32,
    pub border_width: f32,
    pub border_colour: [f32;4],
}

impl UiTextLabelInner{
    pub fn new() -> Self{
        Self {
            text: String::from("text label"),
            text_size: 12.0,

            visible: true,
            position: [0.0,0.0],
            size: [0.1,0.1],
            text_colour: [1.0,1.0,1.0,1.0],
            background_colour: [0.5,0.5,0.5,1.0],

            corner_radius: 0.0, 
            border_width: 0.0, 
            border_colour: [0.0,0.0,0.0,1.0] 
        }
    }
}

impl Renderable for UiTextLabelInner{
    fn render(&self,render_state: &mut RenderState) -> mlua::Result<()> {
        if !self.visible {return Ok(())}

        let x = self.position[0] * 2.0 - 1.0;
        let y = 1.0 - self.position[1] * 2.0;

        let width = self.size[0] * 2.0;
        let height = self.size[1] * 2.0;

        render_state.draw_text(TextEntry { 
            position: [x,y], 
            size: [width,height], 
            text_size: self.text_size, 
            colour: self.text_colour, 
            text: self.text.clone(), 
        });

        render_state.draw_rect(Rect { 
            position: [x,y],
            size: [width,height],
            colour: self.background_colour, 
            corner_radius: self.corner_radius, 
            border_width: self.border_width,
            border_colour: self.border_colour,
        });

        Ok(())
    }
}