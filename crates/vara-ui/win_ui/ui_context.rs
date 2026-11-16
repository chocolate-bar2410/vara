
#[derive(mlua::UserData)]
pub struct UiContext{

}

#[mlua::userdata_impl]
impl UiContext {
    #[lua(skip)]
    pub fn new() -> Self {
        Self {}
    }
}