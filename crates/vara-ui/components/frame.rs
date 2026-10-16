use crate::components::Renderable;


#[derive(mlua::UserData)]
pub struct UiFrame{

}

#[mlua::userdata_impl]
impl UiFrame {
    
}

impl Renderable for UiFrame{
    fn render(&self,render_state: crate::render_state::RenderState) {
        
    }
}