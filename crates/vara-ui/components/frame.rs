use std::sync::{Arc, Mutex, MutexGuard};

use mlua::{ExternalError, ExternalResult};

use crate::render_state::RenderState;
use crate::components::{Renderable, frame_inner::UiFrameInner};

#[derive(mlua::UserData,Clone)]
pub struct UiFrame{
    #[lua(skip)]
    inner: Arc<Mutex<UiFrameInner>>,
}

#[mlua::userdata_impl]
impl UiFrame {

    #[lua(skip)]
    pub fn new() -> Self{
        Self {
           inner: Arc::new(Mutex::new(UiFrameInner::new())),
        }
    }

    #[lua(skip)]
    pub fn lock(&self) -> mlua::Result<MutexGuard<'_, UiFrameInner>>{
        self.inner.lock().map_err(|_| "frame inner mutex was poisioned".into_lua_err())
    }

    #[lua(setter,name="Visible")]
    fn set_visible(&mut self,visible : bool) -> mlua::Result<()>{
        self.lock()?.visible = visible;

        Ok(())
    }

    #[lua(getter,name="Visible")]
    fn get_visible(&self) -> mlua::Result<bool>{
        Ok(self.lock()?.visible)
    }

    #[lua(setter,name="Size")]
    fn set_size(&mut self,size : [f32;2]) -> mlua::Result<()>{
        self.lock()?.size = size;

        Ok(())
    }

    #[lua(getter,name="Size")]
    fn get_size(&self) -> mlua::Result<[f32;2]>{
        Ok(self.lock()?.size)
    }

    #[lua(setter,name="Position")]
    fn set_position(&mut self,position : [f32;2]) -> mlua::Result<()>{
        self.lock()?.position = position;

        Ok(())
    }

    #[lua(getter,name="Position")]
    fn get_position(&self) -> mlua::Result<[f32;2]>{
        Ok(self.lock()?.position)
    }

    #[lua(setter,name="Colour")]
    fn set_colour(&mut self,colour : [f32;4]) -> mlua::Result<()>{
        self.lock()?.colour = colour;

        Ok(())
    }

    #[lua(getter,name="Colour")]
    fn get_colour(&self) -> mlua::Result<[f32;4]>{
        Ok(self.lock()?.colour)
    }

    #[lua(setter,name="CornerRadius")]
    fn set_corner_radius(&mut self,corner_radius : f32) -> mlua::Result<()>{
        self.lock()?.corner_radius = corner_radius;

        Ok(())
    }

    #[lua(getter,name="CornerRadius")]
    fn get_corner_radius(&self) -> mlua::Result<f32>{
        Ok(self.lock()?.corner_radius)
    }

    #[lua(setter,name="BorderWidth")]
    fn set_border_width(&mut self,border_width : f32) -> mlua::Result<()>{
        self.lock()?.border_width = border_width;

        Ok(())
    }

    #[lua(getter,name="BorderWidth")]
    fn get_border_width(&self) -> mlua::Result<f32>{
        Ok(self.lock()?.border_width)
    }


    #[lua(setter,name="BorderColour")]
    fn set_border_colour(&mut self,border_colour : [f32;4]) -> mlua::Result<()>{
        self.lock()?.border_colour = border_colour;

        Ok(())
    }

    #[lua(getter,name="BorderColour")]
    fn get_border_colour(&self) -> mlua::Result<[f32;4]>{
        Ok(self.lock()?.border_colour)
    }


    
}

impl Renderable for UiFrame{
    fn render(&self,render_state: &mut RenderState) -> mlua::Result<()> {
        self.lock()?.render(render_state)?;

        Ok(())
    }
}