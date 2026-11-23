#[derive(mlua::UserData)]
pub struct QtContext{

}

#[mlua::userdata_impl]
impl GtkContext {
    #[lua(skip)]
    pub fn new() -> Self {
        Self{}
    }
}