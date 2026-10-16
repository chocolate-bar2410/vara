use std::sync::{Arc, Mutex};

use mlua::{ExternalError};

use crate::render_state::RenderState;
use crate::components::{Renderable, frame_inner::UiFrameInner};

#[derive(mlua::UserData,Clone)]
pub struct UiFrame{
    #[lua(skip)]
    inner: Arc<Mutex<UiFrameInner>>
}

#[mlua::userdata_impl]
impl UiFrame {

    #[lua(skip)]
    pub fn new() -> Self{
        Self {
           inner: Arc::new(Mutex::new(UiFrameInner::new()))
        }
    }
    
}

impl Renderable for UiFrame{
    fn render(&self,render_state: &mut RenderState) -> mlua::Result<()> {
        let Ok(frame) = self.inner.lock() else{
            return Err("frame inner mutex poisioned".into_lua_err())
        };
    
        frame.render(render_state)?;

        Ok(())
    }
}