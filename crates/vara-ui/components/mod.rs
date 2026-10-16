use frame::UiFrame;

use crate::render_state::RenderState;

pub mod frame;

pub enum UiComponent{
    Frame(UiFrame)
}

pub trait Renderable {
    fn render(&self,render_state: RenderState);
}

impl Renderable for UiComponent {
    fn render(&self,_render_state: RenderState) {}
}