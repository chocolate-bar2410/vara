use std::sync::Arc;

use winit::window::Window;


pub struct RenderState{
    window: Arc<Window>
}

impl RenderState {
    pub async fn new(window: Arc<Window>) -> mlua::Result<Self>{
        Ok(Self { window })
    }

    pub fn resize(&mut self,_height: u32,_width: u32){

    }

    pub fn render(&mut self){
        self.window.request_redraw();
    }
}