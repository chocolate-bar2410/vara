use frame::UiFrame;

use crate::render_state::RenderState;

pub mod frame;
pub mod frame_inner;

pub enum UiComponent{
    Frame(UiFrame)
}

pub trait Renderable {
    fn render(&self,render_state: &mut RenderState) -> mlua::Result<()>;
}

impl Renderable for UiComponent {
    fn render(&self,render_state: &mut RenderState) -> mlua::Result<()>  {
        match self {
            UiComponent::Frame(frame) => frame.render(render_state)
        }
    }
}