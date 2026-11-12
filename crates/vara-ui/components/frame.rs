use std::sync::{Arc, Mutex, MutexGuard};

use mlua::{ExternalError};

use crate::components::ui_property;
use crate::render_state::RenderState;
use crate::components::{Renderable, frame_inner::UiFrameInner};

#[derive(Clone)]
pub struct UiFrame{
    inner: Arc<Mutex<UiFrameInner>>,
}

impl UiFrame {

    pub fn new() -> Self{
        Self {
           inner: Arc::new(Mutex::new(UiFrameInner::new())),
        }
    }

    pub fn lock(&self) -> mlua::Result<MutexGuard<'_, UiFrameInner>>{
        self.inner.lock().map_err(|_| "frame inner mutex was poisoned".into_lua_err())
    }
}

impl mlua::UserData for UiFrame {
    fn add_fields<F: mlua::prelude::LuaUserDataFields<Self>>(fields: &mut F) {
        
        ui_property!(fields,visible,"Visible");
        ui_property!(fields,size,"Size");
        ui_property!(fields,position,"Position");
        ui_property!(fields,colour,"Colour");
        ui_property!(fields,corner_radius,"CornerRadius");
        ui_property!(fields,border_width,"BorderWidth");
        ui_property!(fields,border_colour,"BorderColour");
    }
}

impl Renderable for UiFrame{
    fn render(&self,render_state: &mut RenderState) -> mlua::Result<()> {
        self.lock()?.render(render_state)?;

        Ok(())
    }
}