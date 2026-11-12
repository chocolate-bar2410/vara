use std::sync::{Arc, Mutex, MutexGuard};
use mlua::ExternalError;

use crate::components::{Renderable, ui_property};
use crate::components::textlabel_inner::UiTextLabelInner;
use crate::render_state::RenderState;

#[derive(Clone)]
pub struct UiTextLabel{
    inner: Arc<Mutex<UiTextLabelInner>>,
}

impl UiTextLabel{
    pub fn new() -> Self{
        Self {
           inner: Arc::new(Mutex::new(UiTextLabelInner::new())),
        }
    }

    pub fn lock(&self) -> mlua::Result<MutexGuard<'_, UiTextLabelInner>>{
        self.inner.lock().map_err(|_| "frame inner mutex was poisioned".into_lua_err())
    }
}


impl mlua::UserData for UiTextLabel{
    fn add_fields<F: mlua::prelude::LuaUserDataFields<Self>>(fields: &mut F) {
        
        ui_property!(fields,text,"Text");
        ui_property!(fields,text_size,"TextSize");

        ui_property!(fields,visible,"Visible");
        ui_property!(fields,size,"Size");
        ui_property!(fields,position,"Position");
        ui_property!(fields,text_colour,"TextColour");
        ui_property!(fields,background_colour,"BackgroundColour");

        ui_property!(fields,corner_radius,"CornerRadius");
        ui_property!(fields,border_width,"BorderWidth");
        ui_property!(fields,border_colour,"BorderColour");

    }
}


impl Renderable for UiTextLabel{
    fn render(&self,render_state: &mut RenderState) -> mlua::Result<()> {
       self.lock()?.render(render_state)
    }
}