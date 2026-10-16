use frame::UiFrame;

use crate::{components::textlabel::UiTextLabel, render_state::RenderState};

pub mod frame;
pub mod frame_inner;

pub mod textlabel;
pub mod textlabel_inner;

pub enum UiComponent{
    Frame(UiFrame),
    TextLabel(UiTextLabel)
}

pub trait Renderable {
    fn render(&self,render_state: &mut RenderState) -> mlua::Result<()>;
}

impl Renderable for UiComponent {
    fn render(&self,render_state: &mut RenderState) -> mlua::Result<()>  {
        match self {
            UiComponent::Frame(frame) => frame.render(render_state),
            UiComponent::TextLabel(text) => text.render(render_state)
        }
    }
}

macro_rules! ui_property{
    ($fields:ident,$field:ident,$name:literal) => {
        $fields.add_field_method_set($name, |_, this, value| {
            this.lock()?.$field = value;
            Ok(())
        });

        $fields.add_field_method_get($name, |_: &mlua::Lua, this| {
            Ok(this.lock()?.$field.clone())
        });
    }
}

pub(crate) use ui_property;