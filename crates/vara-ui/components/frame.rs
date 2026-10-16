use std::sync::{Arc, Mutex, MutexGuard};

use mlua::{ExternalError};

use crate::render_state::RenderState;
use crate::components::{Renderable, frame_inner::UiFrameInner};


macro_rules! ui_property{
    ($fields:ident,$field:ident,$name:literal,$ty:ty) => {
        $fields.add_field_method_set($name, |_, this, value: $ty| {
            this.lock()?.$field = value;
            Ok(())
        });

        $fields.add_field_method_get($name, |_: &mlua::Lua, this| {
            Ok(this.lock()?.$field)
        });
    }
}


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
        self.inner.lock().map_err(|_| "frame inner mutex was poisioned".into_lua_err())
    }
}

impl mlua::UserData for UiFrame {
    fn add_fields<F: mlua::prelude::LuaUserDataFields<Self>>(fields: &mut F) {
        
        ui_property!(fields,visible,"Visible",bool);
        ui_property!(fields,size,"Size",[f32;2]);
        ui_property!(fields,position,"Position",[f32;2]);
        ui_property!(fields,colour,"Colour",[f32;4]);
        ui_property!(fields,corner_radius,"CornerRadius",f32);
        ui_property!(fields,border_width,"BorderWidth",f32);
        ui_property!(fields,border_colour,"BorderColour",[f32;4]);
    }
}

impl Renderable for UiFrame{
    fn render(&self,render_state: &mut RenderState) -> mlua::Result<()> {
        self.lock()?.render(render_state)?;

        Ok(())
    }
}